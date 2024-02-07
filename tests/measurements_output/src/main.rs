use canbench::{bench, bench_fn, bench_scope, BenchResult};

#[link(wasm_import_module = "ic0")]
extern "C" {
    pub fn stable64_grow(additional_pages: u64) -> i64;
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that no change is reported.
#[bench]
fn no_changes_test() {}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that a noisy change is reported.
#[bench]
fn noisy_change_test() {}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that regression is reported.
#[bench]
fn regression_test() {}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that an improvement is reported.
#[bench]
fn improvement_test() {}

// The values of the benchmark are persisted such that a regression from zero
// is reported.
#[bench]
fn stable_memory_increase_from_zero() {
    unsafe { stable64_grow(123) };
}

// A benchmark to check that only the _increase_ in stable memory is reported, not
// the total stable memory.
#[bench(raw)]
fn stable_memory_only_increase() -> BenchResult {
    unsafe { stable64_grow(123) };

    // Since only the increase is reported, the benchmark should return an increase
    // of 456 (and ignore the stable memory allocation above).
    bench_fn(|| unsafe { stable64_grow(456) })
}

// A benchmark where we allocate some memory on the heap to increase the heap increase.
#[bench]
fn increase_heap_increase() {
    let _ = vec![1; 1_000_000];
}

// A benchmark that includes some profiling, but isn't persisted in the results.
#[bench]
fn bench_scope_new() {
    {
        let _p = bench_scope("scope_1");
        println!("do something");
    }

    {
        let _p = bench_scope("scope_2");
        println!("do something else");
    }
}

// A benchmark that includes some profiling and is persisted in the results.
#[bench]
fn bench_scope_exists() {
    {
        let _p = bench_scope("scope_1");
        println!("do something");
    }

    {
        let _p = bench_scope("scope_2");
        println!("do something else");
    }
}

fn main() {}
