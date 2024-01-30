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
            "Couldn't read file at ./wasm.wasm. Are you sure the file exists?\n"
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
  total:
    instructions: 310 (no change)
    heap_delta: 0 (no change)
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
  total:
    instructions: 310 (1.64%) (change within noise threshold)
    heap_delta: 0 (no change)
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
  total:
    instructions: 310 (regressed by 3000.00%)
    heap_delta: 0 (no change)
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
  total:
    instructions: 310 (improved by 90.00%)
    heap_delta: 0 (no change)
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
  total:
    instructions: 410 (regressed from 0)
    heap_delta: 0 (no change)
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
  total:
    instructions: 410 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 456 (new)

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
  total:
    instructions: 3385716 (new)
    heap_delta: 62 (new)
    stable_memory_delta: 0 (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn supports_gzipped_wasm() {
    BenchTest::canister("gzipped_wasm").run(|output| {
        assert_success!(
            output,
            "
---------------------------------------------------

Benchmark: bench_1 (new)
  total:
    instructions: 310 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

---------------------------------------------------

Benchmark: bench_2 (new)
  total:
    instructions: 310 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

---------------------------------------------------
"
        );
    });
}

#[test]
fn reports_profiling_in_new_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("profiling_new")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: profiling_new (new)
  total:
    instructions: 3510 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

  step_1 (profiling):
    instructions: 288 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

  step_2 (profiling):
    instructions: 288 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn reports_profiling_in_existing_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("profiling_exists")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: profiling_exists
  total:
    instructions: 3510 (regressed from 0)
    heap_delta: 0 (no change)
    stable_memory_delta: 0 (no change)

  step_1 (profiling):
    instructions: 288 (improved by 64.00%)
    heap_delta: 0 (improved by 100.00%)
    stable_memory_delta: 0 (no change)

  step_2 (profiling):
    instructions: 288 (new)
    heap_delta: 0 (new)
    stable_memory_delta: 0 (new)

---------------------------------------------------
"
            );
        });
}
