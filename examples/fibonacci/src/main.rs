use canbench::bench;

// A version of fibonacci that's efficient.
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}

// Try this inefficient version instead and run `canbench`.
// `canbench` will detect and report the regression.
/* fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}*/

#[bench]
fn fibonacci_20() {
    // NOTE: the result is printed to prevent the compiler from optimizing the call away.
    println!("{:?}", fibonacci(20));
}

fn main() {}
