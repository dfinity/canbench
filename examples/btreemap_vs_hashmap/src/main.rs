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
    // TIP: try replace the `BTreeMap` below with a `HashMap` and run `canbench`.
    // Notice how the performance changes.
    users: std::collections::BTreeMap<u64, User>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize state.
    let bytes = {
        let _p = canbench::profile("serialize_state"); // for profiling.
        STATE.with(|s| Encode!(s).unwrap())
    };

    // Write to stable memory.
    let _p = canbench::profile("writing_to_stable_memory"); // for profiling.
    ic_cdk::api::stable::StableWriter::default()
        .write(&bytes)
        .unwrap();
}

// Benchmarks inserting 1 million users into the state.
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

// Benchmarks removing 1 million users from the state.
#[bench(raw)]
fn remove_users() -> canbench::BenchResult {
    insert_users();

    canbench::benchmark(|| {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            for i in 0..1_000_000 {
                s.users.remove(&i);
            }
        })
    })
}

#[bench(raw)]
fn pre_upgrade_bench() -> canbench::BenchResult {
    insert_users();

    canbench::benchmark(|| pre_upgrade())
}

fn main() {}
