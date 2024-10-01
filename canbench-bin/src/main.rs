//! A script for running benchmarks on a canister.
//! To run this script, run `cargo bench`.
use clap::Parser;
use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf, process::Command};

const CFG_FILE_NAME: &str = "canbench.yml";
const DEFAULT_RESULTS_FILE: &str = "canbench_results.yml";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If provided, only benchmarks that match this pattern will be executed.
    pattern: Option<String>,

    /// Whether or not results should be persisted to disk.
    #[clap(long)]
    persist: bool,

    /// Only print the benchmark results (and nothing else).
    #[clap(long)]
    less_verbose: bool,

    /// Skip checking the integrity (hash) of the runtime.
    ///
    /// Checking the hash ensures that canbench is using the runtime it's expecting.
    /// Only enable this flag if you explicitly want to run canbench using a different
    /// runtime.
    ///
    /// NOTE: canbench can report different benchmark numbers if used with a different runtime.
    #[clap(long)]
    no_runtime_integrity_check: bool,

    /// A path to the runtime to use.
    /// Defaults to `.canbench/pocket-ic`.
    #[clap(long)]
    runtime_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct InitArgs {
    // hex encoded argument to pass to the canister
    hex: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    // If provided, instructs canbench to build the canister
    build_cmd: Option<String>,

    // Where to find the wasm to be benchmarked
    wasm_path: String,

    // If provided, instructs canbench to store the results in this file
    // Otherwise, `canbench_results.yml` is used by default
    results_path: Option<String>,

    // If provided, the init arguments to pass to the canister
    init_args: Option<InitArgs>,
}

// Path to the canbench directory where we keep internal data.
fn canbench_dir() -> PathBuf {
    PathBuf::new()
        .join(std::env::current_dir().unwrap())
        .join(".canbench")
}

// Default path to the runtime (PocketIC)
fn default_runtime_path() -> PathBuf {
    canbench_dir().join("pocket-ic")
}

fn main() {
    let args = Args::parse();

    // Read and parse the configuration file.
    let mut file = match File::open(CFG_FILE_NAME) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("{} not found in current directory.", CFG_FILE_NAME)
                }
                other => println!("Error while opening `{}`: {}", CFG_FILE_NAME, other),
            }

            std::process::exit(1);
        }
    };

    let mut config_str = String::new();
    file.read_to_string(&mut config_str).unwrap();
    let cfg: Config = serde_yaml::from_str(&config_str).unwrap();

    let wasm_path = PathBuf::from(&cfg.wasm_path);
    let results_path = PathBuf::from(
        cfg.results_path
            .as_ref()
            .unwrap_or(&DEFAULT_RESULTS_FILE.to_string()),
    );

    // Build the canister if a build command is specified.
    if let Some(build_cmd) = cfg.build_cmd {
        assert!(
            Command::new("bash")
                .arg("-c")
                .arg(build_cmd)
                .status()
                .unwrap()
                .success(),
            "failed to unwrap build command"
        );
    }

    let init_args = cfg
        .init_args
        .map(|args| hex::decode(args.hex).expect("invalid init_args hex value"))
        .unwrap_or_default();

    // Run the benchmarks.
    canbench::run_benchmarks(
        &wasm_path,
        args.pattern,
        init_args,
        args.persist,
        &results_path,
        !args.less_verbose,
        !args.no_runtime_integrity_check,
        &args.runtime_path.unwrap_or_else(default_runtime_path),
    );
}
