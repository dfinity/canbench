mod utils;
use utils::BenchTest;

#[test]
fn no_config_prints_error() {
    BenchTest::no_config().run(|output| {
        assert_err!(output, "canbench.yml not found in current directory.\n");
    });
}

#[test]
fn wasm_path_incorrect_prints_error() {
    BenchTest::with_config(
        "
wasm_path:
  ./wasm.wasm",
    )
    .run(|output| {
        assert_err!(
            output,
            "Couldn't read wasm file at ./wasm.wasm. Are you sure the file exists?\n"
        );
    });
}

// Runs a benchmark whose measurements were persisted previously, and thus reports no changes.
#[test]
fn benchmark_reports_no_changes() {
    BenchTest::canister("measurements_output")
        .with_bench("no_changes_test")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: no_changes_test
  instructions: 304 (Δ 0.00%) (no change)
  stable_memory_size: 0 (no change)

---------------------------------------------------

Executed 1 of 3 benchmarks.
"
            );
        });
}


#[test]
fn benchmark_reports_regression() {
    BenchTest::canister("measurements_output")
        .with_bench("regression_test")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: regression_test
  instructions: 304 (regressed by 2940.00%)
  stable_memory_size: 0 (no change)

---------------------------------------------------

Executed 1 of 3 benchmarks.
"
            );
        });
}

#[test]
fn benchmark_reports_improvement() {
    BenchTest::canister("measurements_output")
        .with_bench("improvement_test")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: improvement_test
  instructions: 304 (improved by 90.00%)
  stable_memory_size: 0 (no change)

---------------------------------------------------

Executed 1 of 3 benchmarks.
"
            );
        });
}
