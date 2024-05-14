use canbench_rs::bench;
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<String> = RefCell::new(String::new());
}
// A benchmark that prints the state.
#[bench]
fn state_check() {
    let state = STATE.with(|s| s.borrow().clone());
    assert_eq!(state, "hello");
}
#[ic_cdk_macros::init]
pub fn init(arg: String) {
    STATE.with(|s| *s.borrow_mut() = arg);
}
fn main() {}
