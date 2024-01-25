use canbench::{benchmark, macros::bench, BenchResult};

// A benchmark that does nothing.
// The values of the benchmark are persisted in the results file.
#[bench]
fn nothing() -> BenchResult {
    benchmark(|| {})
}

fn main() {}
