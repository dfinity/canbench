use canbench_rs::bench;
use ic_cdk::api::env_var_value;
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

// A benchmark that verifies the expected environment variable values were loaded.
#[bench]
fn state_check() {
    let state = STATE.with(|s| s.borrow().clone());
    assert_eq!(state[0], "value1 with_more_text");
    assert_eq!(state[1], "value2");
    assert_eq!(state[2], "value3");
}

#[ic_cdk::init]
pub fn init() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.push(env_var_value("name1"));
        state.push(env_var_value("name2"));
        state.push(env_var_value("name3"));
    });
}

fn main() {}
