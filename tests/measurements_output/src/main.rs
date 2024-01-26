use canbench::{benchmark, macros::bench, BenchResult};

// A benchmark that does nothing.
// The values of the benchmark are persisted such that no change is reported.
#[bench]
fn no_changes_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that a noisy change is reported.
#[bench]
fn noisy_change_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that regression is reported.
#[bench]
fn regression_test() -> BenchResult {
    benchmark(|| {})
}

// A benchmark that does nothing.
// The values of the benchmark are persisted such that an improvement is reported.
#[bench]
fn improvement_test() -> BenchResult {
    benchmark(|| {})
}

fn main() {}
