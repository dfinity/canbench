#[ic_cdk::query]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[canbench::bench]
fn fibonacci_20() {
    println!("{:?}", fibonacci(20));
}

#[canbench::bench]
fn fibonacci_45() {
    println!("{:?}", fibonacci(45));
}

fn main() {}
