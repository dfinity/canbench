mod utils;
use tempfile::NamedTempFile;
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
    instructions: 207 (no change)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

---------------------------------------------------
"
            );
        });
}

#[test]
fn broken_benchmark_returns_full_error() {
    BenchTest::canister("measurements_output")
        .with_bench("broken_benchmark")
        .run(|output| {
            assert_err!(
                output,
                "Error executing benchmark broken_benchmark. Error:
IC0506: Canister lxzze-o7777-77777-aaaaa-cai did not produce a response
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
    instructions: 207 (-1.43%) (change within noise threshold)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

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
    instructions: 207 (regressed by 1970.00%)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

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
    instructions: 207 (improved by 93.32%)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_regression_from_zero() {
    BenchTest::canister("measurements_output")
        .with_bench("stable_memory_increase_from_zero")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: stable_memory_increase_from_zero
  total:
    instructions: 307 (regressed from 0)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 123 pages (regressed from 0)

---------------------------------------------------
"
            );
        });
}

// Tests that only the stable memory increase is reported (as opposed to the entire
// stable memory usage.
#[test]
fn benchmark_stable_memory_increase() {
    BenchTest::canister("measurements_output")
        .with_bench("stable_memory_only_increase")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: stable_memory_only_increase (new)
  total:
    instructions: 307 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 456 pages (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_heap_increase() {
    BenchTest::canister("measurements_output")
        .with_bench("increase_heap_increase")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: increase_heap_increase (new)
  total:
    instructions: 3.39 M (new)
    heap_increase: 62 pages (new)
    stable_memory_increase: 0 pages (new)

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
    instructions: 207 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: bench_2 (new)
  total:
    instructions: 207 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

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
    instructions: 3411 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

  scope_1 (scope):
    instructions: 1002 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

  scope_2 (scope):
    instructions: 787 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn specifying_a_bogus_runtime_triggers_a_redownload() {
    // Create an empty file and pass it as the runtime.
    // Given that this file's digest doesn't match what canbench expects, it should fail.
    let runtime_file = NamedTempFile::new().unwrap();
    let runtime_path = runtime_file.path().to_path_buf();

    BenchTest::with_config(
        "
wasm_path:
  ./wasm.wasm",
    )
    .with_runtime_path(runtime_path.clone())
    .run(|output| {
        assert_err!(output.clone(), "Runtime has incorrect digest");
        assert_err!(output, "Runtime will be redownloaded");

        // Verify that the runtime has been redownloaded and now has the correct digest.
        let digest = sha256::try_digest(runtime_path).unwrap();

        assert_eq!(digest, canbench::expected_runtime_digest());
    });
}

#[test]
fn specifying_a_bogus_runtime_without_integrity_check() {
    // Create an empty file and pass it as the runtime.
    let runtime_file = NamedTempFile::new().unwrap();
    let runtime_path = runtime_file.path().to_path_buf();

    // Since the runtime integrity check is skipped, canbench won't report
    // a bad digest for the runtime, but will instead report that it can't
    // find the wasm.
    BenchTest::with_config(
        "
wasm_path:
  ./wasm.wasm",
    )
    .with_runtime_path(runtime_path)
    .with_no_runtime_integrity_check()
    .run(|output| {
        assert_err!(output, "Couldn't read file at ./wasm.wasm.");
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
    instructions: 3411 (regressed from 0)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

  scope_1 (scope):
    instructions: 1002 (regressed by 25.25%)
    heap_increase: 0 pages (improved by 100.00%)
    stable_memory_increase: 0 pages (no change)

  scope_2 (scope):
    instructions: 787 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------
"
            );
        });
}

#[test]
fn newer_version() {
    BenchTest::canister("newer_version")
        .run(|output| {
        assert_err!(
                output,
                "canbench is at version 0.1.7 while the results were generated with version 99.0.0. Please upgrade canbench.
"
            );
        });
}

#[test]
fn benchmark_works_with_init_args() {
    BenchTest::canister("init_arg")
        .with_bench("state_check")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: state_check
  total:
    instructions: 804 (no change)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

---------------------------------------------------
"
            );
        });
}

// Ensures writes to stable memory are accounted for in the same way as application subnets.
#[test]
fn benchmark_stable_writes() {
    BenchTest::canister("measurements_output")
        .with_bench("write_stable_memory")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: write_stable_memory (new)
  total:
    instructions: 49.09 K (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 1 pages (new)

---------------------------------------------------
"
            );
        });
}
