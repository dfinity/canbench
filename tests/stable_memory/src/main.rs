use canbench_rs::bench;
use std::cell::RefCell;

thread_local! {
    static DATA: RefCell<Vec<u8>> = RefCell::new(vec![1; 10]);
}

#[ic_cdk::update]
fn bench_setup() {
    // There should be one page in stable memory.
    assert_eq!(ic_cdk::api::stable::stable_size(), 1);

    // Store stable memory data into DATA
    DATA.with(|data| {
        // The `stable_memory.bin` specified in canbench.yml only has the first give bytes set.
        // The rest should be zero.
        ic_cdk::api::stable::stable_read(0, data.borrow_mut().as_mut());
    });
}

#[bench]
fn read_from_stable_memory() {
    let mut buf = [0; 10];
    ic_cdk::api::stable::stable_read(0, &mut buf);

    // Assert that we can read the data loaded by the setup endpoint.
    DATA.with(|data| {
        assert_eq!(
            data.borrow().as_slice(),
            &[0x41, 0x42, 0x43, 0x44, 0x45, 0, 0, 0, 0, 0]
        );
    });
}

fn main() {}
