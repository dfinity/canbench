use canbench_rs::bench;

// A benchmark that does nothing.
#[bench]
fn no_changes_test() {}

#[ic_cdk_macros::init]
pub fn init(arg: String) {
    assert_eq!(arg, "hello".to_string());
}

fn main() {}
