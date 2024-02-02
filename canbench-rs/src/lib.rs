//! `canbench` is a tool for benchmarking canisters on the Internet Computer.
//!
//! ## Quickstart
//! 
//! This example is also available to tinker with in the examples directory. See the [fibonacci example](https://github.com/dfinity/bench/tree/main/examples/fibonacci).
//!
//! ### 1. Install the `canbench` binary.
//!
//! The `canbench` is what runs your canister's benchmarks.
//!
//! ```bash
//! cargo install canbench
//! ```
//! 
//! ### 2. Add optional dependency to `Cargo.toml`
//! 
//! Typically you do not want your benchmarks to be part of your canister when deploying it to the Internet Computer.
//! Therefore, we include `canbench` only as an optional dependency so that it's only included when running benchmarks.
//! For more information about optional dependencies, you can read more about them [here](https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies).
//! 
//! ```toml
//! canbench = { version = "x.y.z", optional = true }
//! ```
//! 
//! ### 3. Add a configuration to `canbench.yml`
//! 
//! The `canbench.yml` configuration file tells `canbench` how to build and run you canister.
//! Below is a typical configuration.
//! Note that we're compiling the canister with the `canbench` feature so that the benchmarking logic is included in the Wasm.
//! 
//! ```yml
//! build_cmd:
//!   cargo build --release --target wasm32-unknown-unknown --features canbench
//! 
//! wasm_path:
//!   ./target/wasm32-unknown-unknown/release/<YOUR_CANISTER>.wasm
//! ```
//! 
//! ### 4. Start benching! 🏋🏽
//! 
//! Let's say we have a canister that exposes a `query` computing the fibonacci sequence of a given number.
//! Here's what that query can look like:
//! 
//! ```rust
//! #[ic_cdk::query]
//! fn fibonacci(n: u32) -> u32 {
//!     if n == 0 {
//!         return 0;
//!     } else if n == 1 {
//!         return 1;
//!     }
//! 
//!     let mut a = 0;
//!     let mut b = 1;
//!     let mut result = 0;
//! 
//!     for _ in 2..=n {
//!         result = a + b;
//!         a = b;
//!         b = result;
//!     }
//! 
//!     result
//! }
//! ```
//! 
//! Now, let's add some benchmarks to this query:
//! 
//! ```rust
//! #[cfg(feature = "canbench")]
//! mod benches {
//!     use super::*;
//!     use canbench::bench;
//!
//!     # fn fibonacci(_: u32) -> u32 { 0 }
//! 
//!     #[bench]
//!     fn fibonacci_20() {
//!         // NOTE: the result is printed to prevent the compiler from optimizing the call away.
//!         println!("{:?}", fibonacci(20));
//!     }
//! 
//!     #[bench]
//!     fn fibonacci_45() {
//!         // NOTE: the result is printed to prevent the compiler from optimizing the call away.
//!         println!("{:?}", fibonacci(45));
//!     }
//! }
//! ```
//! 
//! Run `canbench`. You'll see an output that looks similar to this:
//! 
//! ```txt
//! $ canbench
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_20 (new)
//!   total:
//!     instructions: 2301 (new)
//!     heap_delta: 0 pages (new)
//!     stable_memory_delta: 0 pages (new)
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_45 (new)
//!   total:
//!     instructions: 3088 (new)
//!     heap_delta: 0 pages (new)
//!     stable_memory_delta: 0 pages (new)
//! 
//! ---------------------------------------------------
//! 
//! Executed 2 of 2 benchmarks.
//! ```
//! 
//! ### 5. Track performance regressions
//! 
//! Notice that `canbench` reported the above benchmarks as "new".
//! `canbench` allows you to persist the results of these benchmarks.
//! In subsequent runs, `canbench` reports the performance relative to the last persisted run.
//! 
//! Let's first persist the results above by running `canbench` again, but with the `persist` flag:
//! 
//! ```txt
//! $ canbench --persist
//! ...
//! ---------------------------------------------------
//! 
//! Executed 2 of 2 benchmarks.
//! Successfully persisted results to canbench_results.yml
//! ```
//! 
//! Now, if we run `canbench` again, `canbench` will run the benchmarks, and will additionally report that there were no changes detected in performance.
//! 
//! ```txt
//! $ canbench
//!     Finished release [optimized] target(s) in 0.34s
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_20
//!   total:
//!     instructions: 2301 (no change)
//!     heap_delta: 0 pages (no change)
//!     stable_memory_delta: 0 pages (no change)
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_45
//!   total:
//!     instructions: 3088 (no change)
//!     heap_delta: 0 pages (no change)
//!     stable_memory_delta: 0 pages (no change)
//! 
//! ---------------------------------------------------
//! 
//! Executed 2 of 2 benchmarks.
//! ```
//! 
//! Let's try swapping out our implementation of `fibonacci` with an implementation that's miserably inefficient.
//! Replace the `fibonacci` function defined previously with the following:
//! 
//! ```rust
//! #[ic_cdk::query]
//! fn fibonacci(n: u32) -> u32 {
//!     match n {
//!         0 => 1,
//!         1 => 1,
//!         _ => fibonacci(n - 1) + fibonacci(n - 2),
//!     }
//! }
//! ```
//! 
//! And running `canbench` again, we see that it detects and reports a regression.
//! 
//! ```txt
//! $ canbench
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_20
//!   total:
//!     instructions: 337.93 K (regressed by 14586.14%)
//!     heap_delta: 0 pages (no change)
//!     stable_memory_delta: 0 pages (no change)
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: fibonacci_45
//!   total:
//!     instructions: 56.39 B (regressed by 1826095830.76%)
//!     heap_delta: 0 pages (no change)
//!     stable_memory_delta: 0 pages (no change)
//! 
//! ---------------------------------------------------
//! 
//! Executed 2 of 2 benchmarks.
//! ```
//! 
//! Apparently, the recursive implementation is many orders of magnitude more expensive than the iterative implementation 😱
//! Good thing we found out before deploying this implementation to production.
//! 
//! Notice that `fibonacci_45` took > 50B instructions, which is substantially more than the instruction limit given for a single message execution on the Internet Computer. `canbench` runs benchmarks in an environment that gives them up to 10T instructions.
//! 
//! ## Additional Examples
//! 
//! For the following examples, we'll be using the following canister code, which you can also find in the [examples](./examples/btreemap_vs_hashmap) directory.
//! This canister defines a simple state as well as a `pre_upgrade` function that stores that state into stable memory.
//! 
//! ```rust
//! use candid::{CandidType, Encode};
//! use ic_cdk_macros::pre_upgrade;
//! use std::cell::RefCell;
//! 
//! #[derive(CandidType)]
//! struct User {
//!     name: String,
//! }
//! 
//! #[derive(Default, CandidType)]
//! struct State {
//!     users: std::collections::BTreeMap<u64, User>,
//! }
//! 
//! thread_local! {
//!     static STATE: RefCell<State> = RefCell::new(State::default());
//! }
//! 
//! #[pre_upgrade]
//! fn pre_upgrade() {
//!     // Serialize state.
//!     let bytes = STATE.with(|s| Encode!(s).unwrap());
//! 
//!     // Write to stable memory.
//!     ic_cdk::api::stable::StableWriter::default()
//!         .write(&bytes)
//!         .unwrap();
//! }
//! ```
//! 
//! ### Excluding setup code
//! 
//! Let's say we want to benchmark how long it takes to run the `pre_upgrade` function. We can define the following benchmark:
//! 
//! ```rust
//! #[cfg(feature = "canbench")]
//! mod benches {
//!     use super::*;
//!     use canbench::bench;
//!
//!     # fn initialize_state() {}
//!     # fn pre_upgrade() {}
//! 
//!     #[bench]
//!     fn pre_upgrade_bench() {
//!         // Some function that fills the state with lots of data.
//!         initialize_state();
//! 
//!         pre_upgrade();
//!     }
//! }
//! ```
//! 
//! The problem with the above benchmark is that it's benchmarking both the `pre_upgrade` call _and_ the initialization of the state.
//! What if we're only interested in benchmarking the `pre_upgrade` call?
//! To address this, we can use the `#[bench(raw)]` macro to specify exactly which code we'd like to benchmark.
//! 
//! ```rust
//! #[cfg(feature = "canbench")]
//! mod benches {
//!     use super::*;
//!     use canbench::bench;
//! 
//!     # fn initialize_state() {}
//!     # fn pre_upgrade() {}
//!
//!     #[bench(raw)]
//!     fn pre_upgrade_bench() -> canbench::BenchResult {
//!         // Some function that fills the state with lots of data.
//!         initialize_state();
//! 
//!         // Only benchmark the pre_upgrade. Initializing the state isn't
//!         // included in the results of our benchmark.
//!         canbench::bench_fn(pre_upgrade)
//!     }
//! }
//! ```
//! 
//! Running `canbench` on the example above will benchmark only the code wrapped in `canbench::bench_fn`, which in this case is the call to `pre_upgrade`.
//! 
//! ```txt
//! $ canbench pre_upgrade_bench
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: pre_upgrade_bench (new)
//!   total:
//!     instructions: 717.10 M (new)
//!     heap_delta: 519 pages (new)
//!     stable_memory_delta: 184 pages (new)
//! 
//! ---------------------------------------------------
//! 
//! Executed 1 of 1 benchmarks.
//! ```
//! 
//! ### Granular Benchmarking
//! 
//! Building on the example above, the `pre_upgrade` function does two steps:
//! 
//! 1. Serialize the state
//! 2. Write to stable memory
//! 
//! Suppose we're interested in understanding, within `pre_upgrade`, the resources spent in each of these steps.
//! `canbench` allows you to do more granular benchmarking using the `canbench::bench_scope` function.
//! Here's how we can modify our `pre_upgrade` function:
//! 
//! 
//! ```rust
//! # use candid::{Encode, CandidType};
//! # use ic_cdk_macros::pre_upgrade;
//! # use std::cell::RefCell;
//! #
//! # #[derive(CandidType)]
//! # struct User {
//! #     name: String,
//! # }
//! #
//! # #[derive(Default, CandidType)]
//! # struct State {
//! #     users: std::collections::BTreeMap<u64, User>,
//! # }
//! #
//! # thread_local! {
//! #     static STATE: RefCell<State> = RefCell::new(State::default());
//! # }
//! 
//! #[pre_upgrade]
//! fn pre_upgrade() {
//!     // Serialize state.
//!     let bytes = {
//!         #[cfg(feature = "canbench")]
//!         let _p = canbench::bench_scope("serialize_state");
//!         STATE.with(|s| Encode!(s).unwrap())
//!     };
//! 
//!     // Write to stable memory.
//!     #[cfg(feature = "canbench")]
//!     let _p = canbench::bench_scope("writing_to_stable_memory");
//!     ic_cdk::api::stable::StableWriter::default()
//!         .write(&bytes)
//!         .unwrap();
//! }
//! ```
//! 
//! In the code above, we've asked `canbench` to profile each of these steps separately.
//! Running `canbench` now, each of these steps are reported.
//! 
//! ```txt
//! $ canbench pre_upgrade_bench
//! 
//! ---------------------------------------------------
//! 
//! Benchmark: pre_upgrade_bench (new)
//!   total:
//!     instructions: 717.11 M (new)
//!     heap_delta: 519 pages (new)
//!     stable_memory_delta: 184 pages (new)
//! 
//!   serialize_state (profiling):
//!     instructions: 717.10 M (new)
//!     heap_delta: 519 pages (new)
//!     stable_memory_delta: 0 pages (new)
//! 
//!   writing_to_stable_memory (profiling):
//!     instructions: 502 (new)
//!     heap_delta: 0 pages (new)
//!     stable_memory_delta: 184 pages (new)
//! 
//! ---------------------------------------------------
//! 
//! Executed 1 of 1 benchmarks.
//! ```
pub use canbench_macros::bench;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static SCOPES: RefCell<BTreeMap<&'static str, Measurement>> = RefCell::new(BTreeMap::new());
}

/// The results of a benchmark.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType)]
pub struct BenchResult {
    /// A measurement for the entire duration of the benchmark.
    pub total: Measurement,

    /// Measurements for scopes.
    #[serde(default)]
    pub scopes: BTreeMap<String, Measurement>,
}

/// A benchmark measurement containing various stats.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType, Clone)]
pub struct Measurement {
    /// The number of instructions.
    #[serde(default)]
    pub instructions: u64,

    /// The increase in heap (measured in pages).
    #[serde(default)]
    pub heap_delta: u64,

    /// The increase in stable memory (measured in pages).
    #[serde(default)]
    pub stable_memory_delta: u64,
}

/// Benchmarks the given function.
pub fn bench_fn<R>(f: impl FnOnce() -> R) -> BenchResult {
    reset();
    let start_heap = heap_size();
    let start_stable_memory = ic_cdk::api::stable::stable64_size();
    let start_instructions = instruction_count();
    f();
    let instructions = instruction_count() - start_instructions;
    let stable_memory_delta = ic_cdk::api::stable::stable64_size() - start_stable_memory;
    let heap_delta = heap_size() - start_heap;

    let total = Measurement {
        instructions,
        heap_delta,
        stable_memory_delta,
    };

    let scopes: std::collections::BTreeMap<_, _> = get_scopes_measurements()
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    BenchResult { total, scopes }
}

/// Benchmarks the scope this function is declared in.
///
/// NOTE: It's important to assign this function, otherwise benchmarking won't work correctly.
///
/// # Correct Usage
///
/// ```
/// fn my_func() {
///   let _p = canbench::bench_scope("my_scope");
///   // Do something.
/// }
/// ```
///
/// # Incorrect Usages
///
/// ```
/// fn my_func() {
///   let _ = canbench::bench_scope("my_scope"); // Doesn't capture the scope.
///   // Do something.
/// }
/// ```
///
/// ```
/// fn my_func() {
///   canbench::bench_scope("my_scope"); // Doesn't capture the scope.
///   // Do something.
/// }
/// ```
#[must_use]
pub fn bench_scope(name: &'static str) -> BenchScope {
    BenchScope::new(name)
}

/// An object used for benchmarking a specific scope.
pub struct BenchScope {
    name: &'static str,
    start_instructions: u64,
    start_stable_memory: u64,
    start_heap: u64,
}

impl BenchScope {
    fn new(name: &'static str) -> Self {
        let start_heap = heap_size();
        let start_stable_memory = ic_cdk::api::stable::stable64_size();
        let start_instructions = instruction_count();

        Self {
            name,
            start_instructions,
            start_stable_memory,
            start_heap,
        }
    }
}

impl Drop for BenchScope {
    fn drop(&mut self) {
        let instructions = instruction_count() - self.start_instructions;
        let stable_memory_delta = ic_cdk::api::stable::stable64_size() - self.start_stable_memory;
        let heap_delta = heap_size() - self.start_heap;

        SCOPES.with(|p| {
            let mut p = p.borrow_mut();
            let prev_scope = p.insert(
                self.name,
                Measurement {
                    instructions,
                    heap_delta,
                    stable_memory_delta,
                },
            );

            assert!(
                prev_scope.is_none(),
                "scope {} cannot be specified multiple times.",
                self.name
            );
        });
    }
}

// Clears all scope data.
fn reset() {
    SCOPES.with(|p| p.borrow_mut().clear());
}

// Returns the measurements for any declared scopes.
fn get_scopes_measurements() -> std::collections::BTreeMap<&'static str, Measurement> {
    SCOPES.with(|p| p.borrow().clone())
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
