//! A module for profiling canisters.
use candid::CandidType;
use maplit::btreemap;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

pub use canbench_macros as macros;

thread_local! {
    static PROFILING: RefCell<BTreeMap<&'static str, u64>> = RefCell::new(BTreeMap::new());
}

/// Starts profiling the instructions consumed.
///
/// Instructions are counted and recorded under the given name until the
/// `Profile` object returned is dropped.
pub fn profile(name: &'static str) -> Profile {
    Profile::new(name)
}

/// Clears all profiling data.
pub fn reset() {
    PROFILING.with(|p| p.borrow_mut().clear());
}

/// Returns the number of instructions used for each of the profile names.
pub fn get_results() -> std::collections::BTreeMap<&'static str, u64> {
    PROFILING.with(|p| p.borrow().clone())
}

pub struct Profile {
    name: &'static str,
    start_instructions: u64,
}

impl Profile {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            start_instructions: instruction_count(),
        }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let instructions_count = instruction_count() - self.start_instructions;

        PROFILING.with(|p| {
            let mut p = p.borrow_mut();
            let entry = p.entry(self.name).or_insert(0);
            *entry += instructions_count;
        });
    }
}

fn instruction_count() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        ic_cdk::api::performance_counter(0)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Consider using cpu time here.
        0
    }
}

fn heap_size() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        core::arch::wasm32::memory_size(0) as u64
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

/// The results of a benchmark.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType)]
pub struct BenchResult {
    pub measurements: BTreeMap<String, u64>,
}

/// Benchmarks the given function.
pub fn benchmark<R>(f: impl FnOnce() -> R) -> BenchResult {
    let start_heap = heap_size();
    let start_stable_memory = ic_cdk::api::stable::stable64_size();
    let start_instructions = instruction_count();
    reset();
    f();
    let total_instructions = instruction_count() - start_instructions;
    let stable_memory_delta = ic_cdk::api::stable::stable64_size() - start_stable_memory;
    let heap_delta = heap_size() - start_heap;

    let mut measurements = btreemap! {
        "instructions".to_string() => total_instructions,
        "heap_delta".to_string() => heap_delta,
        "stable_memory_delta".to_string() => stable_memory_delta,
    };

    let mut profiling_results: std::collections::BTreeMap<_, _> = get_results()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    measurements.append(&mut profiling_results);

    BenchResult { measurements }
}
