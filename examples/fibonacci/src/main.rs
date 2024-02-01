
// Try this inefficient version instead and run `canbench`.
// `canbench` will detect and report the regression.
#[ic_cdk::query]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(feature = "canbench")]
mod benches {
    use super::*;
    use canbench::bench;

    #[bench]
    fn fibonacci_20() {
        // NOTE: the result is printed to prevent the compiler from optimizing the call away.
        println!("{:?}", fibonacci(20));
    }

    #[bench]
    fn fibonacci_45() {
        // NOTE: the result is printed to prevent the compiler from optimizing the call away.
        println!("{:?}", fibonacci(45));
    }
}

fn main() {}
