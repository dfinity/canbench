//! A module for running benchmarks.
use canbench_rs::BenchResult;
use candid::Decode;
use flate2::read::GzDecoder;
use std::{
    collections::BTreeMap,
    env,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
mod print_benchmark;
mod results_file;
use print_benchmark::print_benchmark;
use results_file::VersionError;
use wasmparser::Parser as WasmParser;

// The prefix benchmarks are expected to have in their name.
// Other queries exposed by the canister are ignored.
const BENCH_PREFIX: &str = "__canbench__";

/// Runs the benchmarks on the canister available in the provided `canister_wasm_path`.
pub fn run_benchmarks(
    canister_wasm_path: &PathBuf,
    pattern: Option<String>,
    persist: bool,
    results_file: &PathBuf,
    verbose: bool,
) {
    maybe_download_drun(verbose);

    let current_results = match results_file::read(results_file) {
        Ok(current_results) => current_results,
        Err(VersionError {
            our_version,
            their_version,
        }) => {
            eprintln!("canbench is at version {our_version} while the results were generated with version {their_version}. Please upgrade canbench.");
            std::process::exit(1);
        }
    };

    let mut results = BTreeMap::new();
    let mut num_executed_bench_fns = 0;
    let benchmark_fns = extract_benchmark_fns(canister_wasm_path);
    for bench_fn in &benchmark_fns {
        if let Some(pattern) = &pattern {
            if !bench_fn.contains(pattern) {
                continue;
            }
        }

        println!();
        println!("---------------------------------------------------");
        println!();

        let result = run_benchmark(canister_wasm_path, bench_fn);
        print_benchmark(bench_fn, &result, current_results.get(bench_fn));

        results.insert(bench_fn.to_string(), result);
        num_executed_bench_fns += 1;
    }

    println!();
    println!("---------------------------------------------------");

    if verbose {
        println!();
        println!(
            "Executed {num_executed_bench_fns} of {} benchmarks.",
            benchmark_fns.len()
        );
    }

    // Persist the result if requested.
    if persist {
        results_file::write(results_file, results);
        println!(
            "Successfully persisted results to {}",
            results_file.display()
        );
    }
}

// Path to the canbench directory where we keep internal data.
fn canbench_dir() -> PathBuf {
    PathBuf::new()
        .join(env::current_dir().unwrap())
        .join(".canbench")
}

// Path to drun.
fn drun_path() -> PathBuf {
    canbench_dir().join("drun")
}

// Downloads drun if it's not already downloaded.
fn maybe_download_drun(verbose: bool) {
    const DRUN_LINUX_SHA: &str = "182b800a7979e1e3e516e54e4b9980e5407ced7464c0b3aec9ff7af6e9e69a1b";
    const DRUN_MAC_SHA: &str = "8e0d0758d5a5c6f367e2c374dc7eae0106c7f46a3457f81018af6d5159d2dad4";

    if drun_path().exists() {
        // Drun found. Verify that it's the version we expect it to be.
        let expected_sha = match env::consts::OS {
            "linux" => DRUN_LINUX_SHA,
            "macos" => DRUN_MAC_SHA,
            _ => panic!("only linux and macos are currently supported."),
        };

        let drun_sha = sha256::try_digest(drun_path()).unwrap();

        if drun_sha == expected_sha {
            // Shas match. No need to download drun.
            return;
        }
    }

    // The expected version of drun isn't present. Download it.
    download_drun(verbose);
}

fn download_drun(verbose: bool) {
    const DRUN_URL_PREFIX: &str =
        "https://github.com/dfinity/ic/releases/download/release-2024-01-25_14-09/drun-x86_64-";

    if verbose {
        println!("Downloading runtime (will be cached for future uses)...");
    }

    // Create the canbench directory if it doesn't exist.
    std::fs::create_dir_all(canbench_dir()).unwrap();

    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        panic!("Unsupported operating system");
    };

    let url = format!("{}{}.gz", DRUN_URL_PREFIX, os);
    let drun_compressed = reqwest::blocking::get(url)
        .unwrap()
        .bytes()
        .expect("Failed to download drun");

    let mut decoder = GzDecoder::new(&drun_compressed[..]);
    let mut file = File::create(drun_path()).expect("Failed to create drun file");

    std::io::copy(&mut decoder, &mut file).expect("Failed to write drun file");

    // Make the file executable.
    Command::new("chmod")
        .arg("+x")
        .arg(drun_path())
        .status()
        .unwrap();
}

// Runs the given benchmark.
fn run_benchmark(canister_wasm_path: &Path, bench_fn: &str) -> BenchResult {
    // drun is used for running the benchmark.
    // First, we create a temporary file with steps for drun to execute the benchmark.
    let mut temp_file = tempfile::Builder::new().tempfile().unwrap();
    write!(
        temp_file,
        "create
install rwlgt-iiaaa-aaaaa-aaaaa-cai {} \"\"
query rwlgt-iiaaa-aaaaa-aaaaa-cai {}{} \"DIDL\x00\x00\"",
        canister_wasm_path.display(),
        BENCH_PREFIX,
        bench_fn
    )
    .unwrap();

    // Run the benchmark with drun.
    let drun_output = Command::new(drun_path())
        .args(vec![
            temp_file.into_temp_path().to_str().unwrap(),
            "--instruction-limit",
            "99999999999999",
        ])
        .output()
        .unwrap();

    // Extract the hex response.
    let result_hex = String::from_utf8(drun_output.stdout)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .to_string();

    // Decode the response.
    Decode!(
        &hex::decode(&result_hex[2..]).unwrap_or_else(|_| panic!(
            "error parsing result of benchmark {}. Result: {}",
            bench_fn, result_hex
        )),
        BenchResult
    )
    .expect("error decoding benchmark result {:?}")
}

// Extract the benchmarks that need to be run.
fn extract_benchmark_fns(canister_wasm_path: &PathBuf) -> Vec<String> {
    // Parse the canister's wasm.
    let wasm = std::fs::read(canister_wasm_path).unwrap_or_else(|_| {
        eprintln!(
            "Couldn't read file at {}. Are you sure the file exists?",
            canister_wasm_path.display()
        );
        std::process::exit(1);
    });

    // Decompress the wasm if it's gzipped.
    let wasm = match canister_wasm_path.extension().unwrap().to_str() {
        Some("gz") => {
            // Decompress the wasm if it's gzipped.
            let mut decoder = GzDecoder::new(&wasm[..]);
            let mut decompressed_wasm = vec![];
            decoder.read_to_end(&mut decompressed_wasm).unwrap();
            decompressed_wasm
        }
        _ => wasm,
    };

    let prefix = format!("canister_query {BENCH_PREFIX}");

    WasmParser::new(0)
        .parse_all(&wasm)
        .filter_map(|section| match section {
            Ok(wasmparser::Payload::ExportSection(export_section)) => {
                let queries: Vec<_> = export_section
                    .into_iter()
                    .filter_map(|export| {
                        if let Ok(export) = export {
                            if export.name.starts_with(&prefix) {
                                return Some(
                                    export
                                        .name
                                        .split(&prefix)
                                        .last()
                                        .expect("query must have a name.")
                                        .to_string(),
                                );
                            }
                        }

                        None
                    })
                    .collect();

                Some(queries)
            }
            _ => None,
        })
        .flatten()
        .collect()
}
