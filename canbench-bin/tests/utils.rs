use std::{
    env,
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Output},
};
use tempfile::tempdir;

#[macro_export]
macro_rules! assert_err {
    ($output:expr, $err_str:expr) => {
        assert_eq!($output.status.code(), Some(1), "output: {:?}", $output);
        pretty_assertions::assert_eq!(&String::from_utf8($output.stderr).unwrap(), $err_str);
    };
}

#[macro_export]
macro_rules! assert_success {
    ($output:expr, $out_str:expr) => {
        assert_eq!($output.status.code(), Some(0), "output: {:?}", $output);
        pretty_assertions::assert_eq!(&String::from_utf8($output.stdout).unwrap(), $out_str);
    };
}

pub struct BenchTest {
    config: Option<String>,
    bench_name: Option<String>,
    base_dir: BaseDir,
}

impl BenchTest {
    pub fn no_config() -> Self {
        Self {
            config: None,
            bench_name: None,
            base_dir: BaseDir::Temp,
        }
    }

    pub fn with_config(config: &str) -> Self {
        Self {
            config: Some(config.into()),
            bench_name: None,
            base_dir: BaseDir::Temp,
        }
    }

    /// Creates a test that executes the benchmarks of a canister in the `tests` dir.
    pub fn canister(canister_name: &str) -> Self {
        Self {
            config: None,
            bench_name: None,
            base_dir: BaseDir::Path(
                PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("..")
                    .join("tests")
                    .join(canister_name),
            ),
        }
    }

    pub fn with_bench(self, bench_name: &str) -> Self {
        Self {
            bench_name: Some(bench_name.to_string()),
            ..self
        }
    }

    pub fn run<R>(self, f: impl FnOnce(Output) -> R) {
        let canbench: &'static str = env!("CARGO_BIN_EXE_canbench");

        // Create a temporary directory in case no specific directory is specified.
        let dir = tempdir().unwrap();

        let dir_path = match &self.base_dir {
            BaseDir::Temp => dir.path(),
            BaseDir::Path(path) => path,
        };

        if let Some(config) = self.config {
            // Write the canbench.yml file with the config provided.
            let config_path = dir_path.join("canbench.yml");
            let mut config_file = File::create(config_path).unwrap();
            config_file.write_all(config.as_bytes()).unwrap()
        }

        // Only output the benchmarks so that the output isn't polluted by other
        // statements (e.g. downloading runtime).
        let mut cmd_args = vec!["--less-verbose".to_string()];
        if let Some(bench_name) = self.bench_name {
            cmd_args.push(bench_name.clone());
        }

        let output = Command::new(canbench)
            .current_dir(dir_path)
            .args(cmd_args)
            .output()
            .unwrap();

        f(output);
    }
}

// The base directory to use for running canbench.
enum BaseDir {
    // A temporary directory is created.
    Temp,

    // A specific path is specified.
    Path(PathBuf),
}
