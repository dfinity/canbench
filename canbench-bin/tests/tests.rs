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
  heap_delta: 0 (no change)
  instructions: 310 (no change)
  stable_memory_delta: 0 (no change)

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_noisy_change() {
    BenchTest::canister("measurements_output")
        .with_bench("noisy_change_test")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: noisy_change_test
  heap_delta: 0 (no change)
  instructions: 310 (1.31%) (change within noise threshold)
  stable_memory_delta: 0 (no change)

---------------------------------------------------
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
  heap_delta: 0 (no change)
  instructions: 310 (regressed by 3000.00%)
  stable_memory_delta: 0 (no change)

---------------------------------------------------
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
  heap_delta: 0 (no change)
  instructions: 310 (improved by 90.00%)
  stable_memory_delta: 0 (no change)

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_regression_from_zero() {
    BenchTest::canister("measurements_output")
        .with_bench("stable_memory_increase")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: stable_memory_increase
  heap_delta: 0 (no change)
  instructions: 410 (regressed from 0)
  stable_memory_delta: 123 (regressed from 0)

---------------------------------------------------
"
            );
        });
}

// Tests that only the stable memory delta is reported (as opposed to the entire
// stable memory usage.
#[test]
fn benchmark_stable_memory_delta() {
    BenchTest::canister("measurements_output")
        .with_bench("stable_memory_delta")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: stable_memory_delta (new)
  heap_delta: 0
  instructions: 410
  stable_memory_delta: 456

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_heap_delta() {
    BenchTest::canister("measurements_output")
        .with_bench("increase_heap_delta")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: increase_heap_delta (new)
  heap_delta: 62
  instructions: 3385672
  stable_memory_delta: 0

---------------------------------------------------
"
            );
        });
}
