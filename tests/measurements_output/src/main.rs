use canbench_rs::{
    bench, bench_fn, bench_scope_id, set_bench_id_resolver, BenchResult, ScopeIdName,
};

#[link(wasm_import_module = "ic0")]
extern "C" {
    pub fn stable64_grow(additional_pages: u64) -> i64;
    pub fn stable64_write(offset: i64, src: i64, size: i64);
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that no change is reported.
#[bench]
fn no_changes_test() {}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that a noisy change is reported.
#[bench]
fn noisy_change_test() {}

// A benchmark that uses large enough number of instructions, heap and stable memory so that cases
// related to noise threshold can be tested. The values of the benchmark are persisted such that a
// noisy change is reported with a higher noise threshold, while significant change is reported with
// the default noise threshold.
#[bench]
fn noisy_change_above_default_threshold_test() {
    let _ = vec![1; 1_000_000];
    unsafe { stable64_grow(100) };
}

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

// A benchmark where some bytes are written to stable memory.
#[bench]
fn write_stable_memory() {
    let v = vec![1; 10_000];

    unsafe {
        stable64_grow(1);
        stable64_write(0, v.as_ptr() as i64, v.len() as i64);
    }
}

#[repr(u16)]
#[derive(Copy, Clone)]
enum ScopeId {
    Something = 0,
    SomethingElse = 1,
}

impl ScopeIdName for ScopeId {
    fn name_from_id(id: u16) -> Option<&'static str> {
        match id {
            0 => Some("Something"),
            1 => Some("SomethingElse"),
            _ => None,
        }
    }
}

// A benchmark that includes some profiling, but isn't persisted in the results.
#[bench]
fn bench_scope_new() {
    set_bench_id_resolver::<ScopeId>();

    {
        let _p = bench_scope_id(ScopeId::Something as u16);
        println!("do something");
    }

    {
        let _p = bench_scope_id(ScopeId::SomethingElse as u16);
        println!("do something else");
    }
}

// A benchmark that includes some profiling and is persisted in the results.
#[bench]
fn bench_scope_exists() {
    set_bench_id_resolver::<ScopeId>();

    {
        let _p = bench_scope_id(ScopeId::Something as u16);
        println!("do something");
    }

    {
        let _p = bench_scope_id(ScopeId::SomethingElse as u16);
        println!("do something else");
    }
}

// A benchmark that includes a repeated scope, but isn't persisted in the results.
#[bench]
fn bench_repeated_scope_new() {
    set_bench_id_resolver::<ScopeId>();

    {
        for _ in 0..10 {
            let _p = bench_scope_id(ScopeId::Something as u16);
            println!("do something");
        }
    }
}

// A benchmark that includes a repeated scope and is persisted in the results.
#[bench]
fn bench_repeated_scope_exists() {
    set_bench_id_resolver::<ScopeId>();

    {
        for _ in 0..10 {
            let _p = bench_scope_id(ScopeId::Something as u16);
            println!("do something");
        }
    }
}

#[export_name = "canister_query __canbench__broken_benchmark"]
fn broken_benchmark() {
    // This benchmark doesn't reply, and will therefore fail.
}

fn main() {}
