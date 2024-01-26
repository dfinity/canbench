use std::{
    fs::File,
    io::Write,
    process::{Command, Output},
};
use tempdir::TempDir;

#[macro_export]
macro_rules! assert_err {
    ($output:expr, $err_str:expr) => {
        assert_eq!($output.status.code(), Some(1));
        assert_eq!($output.stderr, $err_str.as_bytes());
    };
}

pub struct BenchTest {
    config: Option<String>,
}

impl BenchTest {
    pub fn no_config() -> Self {
        Self { config: None }
    }

    pub fn with_config(config: &str) -> Self {
        Self {
            config: Some(config.into()),
        }
    }

    pub fn run<R>(self, f: impl FnOnce(Output) -> R) {
        let canbench: &'static str = env!("CARGO_BIN_EXE_canbench");

        // Create a temporary directory which will contain the config file, wasm file, etc.
        let dir = TempDir::new("").unwrap();
        let dir_path = dir.path();

        if let Some(config) = self.config {
            // Write the canbench.yml file with the config provided.
            let config_path = dir_path.join("canbench.yml");
            let mut config_file = File::create(config_path).unwrap();
            config_file.write_all(config.as_bytes()).unwrap()
        }

        let output = Command::new(canbench)
            .current_dir(dir_path)
            .output()
            .unwrap();

        f(output);
    }
}
