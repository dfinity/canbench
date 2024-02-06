use canbench::BenchResult;
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

/// An error returned if the current version if canbench is older than the
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

    let our_version = Version::parse(VERSION).unwrap();
    let their_version =
        Version::parse(results.version).expect("couldn't parse version in results file");
    if our_version < their_version {
        return Err(VersionError {
            our_version,
            their_version,
        });
    }

    Ok(results.benches)
}

/// Write benchmark results to disk.
pub fn write(results_file: &PathBuf, results: BTreeMap<String, BenchResult>) {
    let persisted_results = PersistedResults {
        version: VERSION,
        benches: results,
    };

    // Open a file in write-only mode.
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
    benches: BTreeMap<String, BenchResult>,
    version: &'b str,
}
