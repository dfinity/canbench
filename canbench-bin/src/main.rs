//! A script for running benchmarks on a canister.
//! To run this script, run `cargo bench`.
use clap::Parser;
use std::{collections::BTreeMap, fs::File, io::Read, path::PathBuf, process::Command};

const CFG_FILE_NAME: &str = "canbench.yml";
const DEFAULT_RESULTS_FILE: &str = "canbench_results.yml";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // If provided, only benchmarks that match this pattern will be executed.
    pattern: Option<String>,

    // Whether or not results should be persisted to disk.
    #[clap(long)]
    persist: bool,

    // If true, only prints the benchmark results (and nothing else).
    #[clap(long)]
    less_verbose: bool,
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
    let cfg: BTreeMap<String, String> = serde_yaml::from_str(&config_str).unwrap();

    let wasm_path = PathBuf::from(
        cfg.get("wasm_path")
            .expect("`wasm_path` in bench.yml must be specified."),
    );

    let results_path = PathBuf::from(
        cfg.get("results_path")
            .unwrap_or(&DEFAULT_RESULTS_FILE.to_string()),
    );

    // Build the canister if a build command is specified.
    if let Some(build_cmd) = cfg.get("build_cmd") {
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

    // Run the benchmarks.
    canbench::run_benchmarks(
        &wasm_path,
        args.pattern,
        args.persist,
        &results_path,
        !args.less_verbose,
    );
}
