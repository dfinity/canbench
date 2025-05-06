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

Summary:
  instructions:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_no_changes_with_hide_results() {
    BenchTest::canister("measurements_output")
        .with_bench("no_changes_test")
        .with_hide_results()
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

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

Summary:
  instructions:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min -3 | med -3 | max -3]
    change % : [min -1.43% | med -1.43% | max -1.43%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_noisy_change_above_default_noise_threshold() {
    BenchTest::canister("measurements_output")
        .with_bench("noisy_change_above_default_threshold_test")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: noisy_change_above_default_threshold_test
  total:
    instructions: 3.39 M (improved by 4.35%)
    heap_increase: 62 pages (improved by 4.62%)
    stable_memory_increase: 100 pages (improved by 3.85%)

---------------------------------------------------

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min -154.07 K | med -154.07 K | max -154.07 K]
    change % : [min -4.35% | med -4.35% | max -4.35%]

  heap_increase:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min -3 | med -3 | max -3]
    change % : [min -4.62% | med -4.62% | max -4.62%]

  stable_memory_increase:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min -4 | med -4 | max -4]
    change % : [min -3.85% | med -3.85% | max -3.85%]

---------------------------------------------------
"
            );
        });
}

#[test]
fn benchmark_reports_noisy_change_within_custom_noise_threshold() {
    BenchTest::canister("measurements_output")
        .with_bench("noisy_change_above_default_threshold_test")
        .with_noise_threshold(5.0)
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: noisy_change_above_default_threshold_test
  total:
    instructions: 3.39 M (-4.35%) (change within noise threshold)
    heap_increase: 62 pages (-4.62%) (change within noise threshold)
    stable_memory_increase: 100 pages (-3.85%) (change within noise threshold)

---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min -154.07 K | med -154.07 K | max -154.07 K]
    change % : [min -4.35% | med -4.35% | max -4.35%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min -3 | med -3 | max -3]
    change % : [min -4.62% | med -4.62% | max -4.62%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min -4 | med -4 | max -4]
    change % : [min -3.85% | med -3.85% | max -3.85%]

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

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 197 | med 197 | max 197]
    change % : [min +1970.00% | med +1970.00% | max +1970.00%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

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

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min -2.89 K | med -2.89 K | max -2.89 K]
    change % : [min -93.32% | med -93.32% | max -93.32%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

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

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 307 | med 307 | max 307]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 123 | med 123 | max 123]
    change % : n/a

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

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 307 | med 307 | max 307]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 456 | med 456 | max 456]
    change % : n/a

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

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 3.39 M | med 3.39 M | max 3.39 M]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 62 | med 62 | max 62]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : n/a

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

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 2, total: 2
    change   : [min 207 | med 207 | max 207]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 2, total: 2
    change   : [min 0 | med 0 | max 0]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 2, total: 2
    change   : [min 0 | med 0 | max 0]
    change % : n/a

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
    instructions: 4626 (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 0 pages (new)

  scope_1 (scope):
    instructions: 1620 (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 0 pages (new)

  scope_2 (scope):
    instructions: 786 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 4.63 K | med 4.63 K | max 4.63 K]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : n/a

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
    instructions: 4626 (regressed from 0)
    heap_increase: 1 pages (regressed from 0)
    stable_memory_increase: 0 pages (no change)

  scope_1 (scope):
    instructions: 1620 (regressed by 102.50%)
    heap_increase: 1 pages (improved by 91.67%)
    stable_memory_increase: 0 pages (no change)

  scope_2 (scope):
    instructions: 786 (new)
    heap_increase: 0 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 4.63 K | med 4.63 K | max 4.63 K]
    change % : n/a

  heap_increase:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

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
                "canbench is at version 0.1.11 while the results were generated with version 99.0.0. Please upgrade canbench.
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
    instructions: 872 (regressed by 3.69%)
    heap_increase: 0 pages (no change)
    stable_memory_increase: 0 pages (no change)

---------------------------------------------------

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 31 | med 31 | max 31]
    change % : [min +3.69% | med +3.69% | max +3.69%]

  heap_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

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
    instructions: 49.74 K (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 1 pages (new)

---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 49.74 K | med 49.74 K | max 49.74 K]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

---------------------------------------------------
"
            );
        });
}

#[test]
fn loads_stable_memory_file() {
    BenchTest::canister("stable_memory").run(|output| {
        // There are assertions in the code of that canister itself, so
        // all is needed is to assert that the run succeeded.
        assert_eq!(output.status.code(), Some(0), "output: {:?}", output);
    });
}

#[test]
fn stable_memory_file_not_exit_prints_error() {
    BenchTest::canister("stable_memory_invalid").run(|output| {
        assert_err!(
            output,
            "
Error reading stable memory file stable_memory_does_not_exist.bin
Error: No such file or directory"
        );
    });
}

#[test]
fn shows_canister_output() {
    BenchTest::canister("debug_print")
        .with_canister_output()
        .run(|output| {
            let err_output = String::from_utf8_lossy(&output.stderr);
            assert!(err_output.contains("Hello from tests!"));
        });
}

#[test]
fn benchmark_instruction_tracing() {
    // TODO: better end-to-end testing, since this test only makes sure there is no error in
    // tracing.
    BenchTest::canister("measurements_output")
        .with_bench("write_stable_memory")
        .with_instruction_tracing()
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: write_stable_memory (new)
  total:
    instructions: 49.74 K (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 1 pages (new)
Instruction traces written to write_stable_memory.svg

---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 49.74 K | med 49.74 K | max 49.74 K]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

---------------------------------------------------
"
            );
        });
}

#[test]
fn reports_repeated_scope_in_new_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("bench_repeated_scope_new")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: bench_repeated_scope_new (new)
  total:
    instructions: 16.97 K (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 0 pages (new)

  scope_1 (scope):
    instructions: 8694 (new)
    heap_increase: 1 pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Summary:
  instructions:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 16.97 K | med 16.97 K | max 16.97 K]
    change % : n/a

  heap_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 0, new: 1, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : n/a

---------------------------------------------------
"
            );
        });
}

#[test]
fn reports_repeated_scope_in_existing_benchmark() {
    BenchTest::canister("measurements_output")
        .with_bench("bench_repeated_scope_exists")
        .run(|output| {
            assert_success!(
                output,
                "
---------------------------------------------------

Benchmark: bench_repeated_scope_exists
  total:
    instructions: 16.97 K (regressed from 0)
    heap_increase: 1 pages (regressed from 0)
    stable_memory_increase: 0 pages (no change)

  scope_1 (scope):
    instructions: 8694 (regressed by 986.75%)
    heap_increase: 1 pages (improved by 91.67%)
    stable_memory_increase: 0 pages (no change)

---------------------------------------------------

Summary:
  instructions:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 16.97 K | med 16.97 K | max 16.97 K]
    change % : n/a

  heap_increase:
    changed: 1, unchanged: 0, new: 0, total: 1
    change   : [min 1 | med 1 | max 1]
    change % : n/a

  stable_memory_increase:
    changed: 0, unchanged: 1, new: 0, total: 1
    change   : [min 0 | med 0 | max 0]
    change % : [min 0% | med 0% | max 0%]

---------------------------------------------------
"
            );
        });
}
