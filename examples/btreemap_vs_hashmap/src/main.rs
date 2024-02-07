use canbench::bench;
use candid::{CandidType, Encode};
use ic_cdk_macros::pre_upgrade;
use std::cell::RefCell;

#[derive(CandidType)]
struct User {
    name: String,
}

#[derive(Default, CandidType)]
struct State {
    users: std::collections::BTreeMap<u64, User>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize state.
    let bytes = STATE.with(|s| Encode!(s).unwrap());

    // Write to stable memory.
    ic_cdk::api::stable::StableWriter::default()
        .write(&bytes)
        .unwrap();
}

#[bench]
fn insert_users() {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        for i in 0..1_000_000 {
            s.users.insert(
                i,
                User {
                    name: "foo".to_string(),
                },
            );
        }
    });
}

fn main() {}
