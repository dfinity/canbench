use canbench_rs::{BenchResult, Measurement};
use candid::CandidType;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// An error returned if the current version of canbench is older than the
/// version used to created the results file.
pub struct VersionError {
    pub our_version: Version,
    pub their_version: Version,
}

/// Read a results file and return the benchmark results.
pub fn read(results_file: &PathBuf) -> Result<BTreeMap<String, BenchResult>, VersionError> {
    // Create a path to the desired file
    let mut file = match File::open(results_file) {
        Err(_) => {
            // No current results found.
            return Ok(BTreeMap::new());
        }
        Ok(file) => file,
    };

    // Read the current results.
    let mut results_str = String::new();
    file.read_to_string(&mut results_str)
        .expect("error reading results file");

    let results: PersistedResults = serde_yaml::from_str(&results_str).unwrap();

    // Validate that our version of canbench is not older than what was used
    // to generate the file.
    let our_version = Version::parse(VERSION).unwrap();
    let their_version =
        Version::parse(results.version).expect("couldn't parse version in results file");
    if our_version < their_version {
        return Err(VersionError {
            our_version,
            their_version,
        });
    }

    Ok(results
        .benches
        .into_iter()
        .map(|(k, v)| (k, BenchResult::from(v)))
        .collect())
}

/// Write benchmark results to disk.
pub fn write(results_file: &PathBuf, benches: BTreeMap<String, BenchResult>) {
    let persisted_results = PersistedResults {
        version: VERSION,
        benches: benches
            .into_iter()
            .map(|(k, v)| (k, BenchResultWire::from(v)))
            .collect(),
    };

    let mut file = File::create(results_file).unwrap();
    file.write_all(
        serde_yaml::to_string(&persisted_results)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}

// Data persisted to a results file.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PersistedResults<'b> {
    benches: BTreeMap<String, BenchResultWire>,
    version: &'b str,
}

/// A wire format for benchmark results.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType, Default)]
pub struct BenchResultWire {
    pub total: MeasurementWire,

    #[serde(default)]
    pub scopes: BTreeMap<String, MeasurementWire>,
}

impl From<BenchResult> for BenchResultWire {
    fn from(br: BenchResult) -> Self {
        Self {
            total: MeasurementWire::from(br.total),
            scopes: br
                .scopes
                .into_iter()
                .map(|(k, v)| (k, MeasurementWire::from(v)))
                .collect(),
        }
    }
}

impl From<BenchResultWire> for BenchResult {
    fn from(br: BenchResultWire) -> Self {
        Self {
            total: Measurement::from(br.total),
            scopes: br
                .scopes
                .into_iter()
                .map(|(k, v)| (k, Measurement::from(v)))
                .collect(),
        }
    }
}

/// A wire format for measurements.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType, Clone, Default)]
pub struct MeasurementWire {
    #[cfg(feature = "calls")]
    pub calls: Option<u64>,
    pub instructions: Option<u64>,
    pub heap_increase: Option<u64>,
    pub stable_memory_increase: Option<u64>,
}

impl From<Measurement> for MeasurementWire {
    fn from(m: Measurement) -> Self {
        Self {
            #[cfg(feature = "calls")]
            calls: Some(m.calls),
            instructions: Some(m.instructions),
            heap_increase: Some(m.heap_increase),
            stable_memory_increase: Some(m.stable_memory_increase),
        }
    }
}

impl From<MeasurementWire> for Measurement {
    fn from(m: MeasurementWire) -> Self {
        Self {
            #[cfg(feature = "calls")]
            calls: m.calls.unwrap_or(0),
            instructions: m.instructions.unwrap_or(0),
            heap_increase: m.heap_increase.unwrap_or(0),
            stable_memory_increase: m.stable_memory_increase.unwrap_or(0),
        }
    }
}

#[test]
fn test_backwards_compatibility() {
    use candid::{Decode, Encode};

    #[derive(Serialize, Deserialize, CandidType)]
    pub struct MeasurementPreviousVersion {
        pub instructions: u64,
        pub heap_increase: u64,
        pub stable_memory_increase: u64,
    }

    // Encode a previous version Candid struct (the fields were not provided)
    let input = MeasurementPreviousVersion {
        instructions: 1,
        heap_increase: 2,
        stable_memory_increase: 3,
    };
    let encoded = Encode!(&input).unwrap();
    let decoded = Decode!(&encoded, MeasurementWire).expect("Failed to decode previous version");
    let result = Measurement::from(decoded);

    assert_eq!(
        result,
        Measurement {
            #[cfg(feature = "calls")]
            calls: 0,
            instructions: 1,
            heap_increase: 2,
            stable_memory_increase: 3,
        }
    );
}
