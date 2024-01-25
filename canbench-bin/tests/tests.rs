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
        .with_bench("nothing")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: nothing
  instructions: 304 (Δ 0.00%) (no change)
  stable_memory_size: 0 (no change)

---------------------------------------------------

Executed 1 of 1 benchmarks.
"
            );
        });
}
