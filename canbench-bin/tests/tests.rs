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
    instructions: 206 (no change)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

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
    instructions: 206 (-1.90%) (change within noise threshold)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

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
    instructions: 206 (regressed by 1960.00%)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

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
    instructions: 206 (improved by 93.35%)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

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
    instructions: 306 (regressed from 0)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 123 pages (regressed from 0)

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
    instructions: 306 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 456 pages (new)

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
    instructions: 3.39 M (new)
    heap_delta: 62 pages (new)
    stable_memory_delta: 0 pages (new)

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
    instructions: 206 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------

Benchmark: bench_2 (new)
  total:
    instructions: 206 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------
"
        );
    });
}

#[test]
fn reports_scopes_in_new_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("bench_scope_new")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: bench_scope_new (new)
  total:
    instructions: 3367 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

  scope_1 (scope):
    instructions: 987 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

  scope_2 (scope):
    instructions: 777 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn reports_scopes_in_existing_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("bench_scope_exist")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: bench_scope_exists
  total:
    instructions: 3367 (regressed from 0)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

  scope_1 (scope):
    instructions: 987 (regressed by 23.38%)
    heap_delta: 0 pages (improved by 100.00%)
    stable_memory_delta: 0 pages (no change)

  scope_2 (scope):
    instructions: 777 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------
"
            );
        });
}
