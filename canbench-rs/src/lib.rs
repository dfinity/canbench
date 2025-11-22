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
//! canbench-rs = { version = "x.y.z", optional = true }
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
//!   cargo build --release --target wasm32-unknown-unknown --locked --features canbench-rs
//!
//! wasm_path:
//!   ./target/wasm32-unknown-unknown/release/<YOUR_CANISTER>.wasm
//! ```
//! #### Init Args
//!
//! Init args can be specified using the `init_args` key in the configuration file:
//! ```yml
//! init_args:
//!   hex: 4449444c0001710568656c6c6f
//! ```
//!
//! #### Stable Memory
//!
//! A file can be specified to be loaded in the canister's stable memory _after_ initialization.
//!
//! ```yml
//! stable_memory:
//!   file:
//!     stable_memory.bin
//! ```
//!
//! <div class="warning">Contents of the stable memory file are loaded <i>after</i> the call to the canister's init method.
//! Therefore, changes made to stable memory in the init method would be overwritten.</div>
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
//! #[cfg(feature = "canbench-rs")]
//! mod benches {
//!     use super::*;
//!     use canbench_rs::bench;
//!
//!     # fn fibonacci(_: u32) -> u32 { 0 }
//!
//!     #[bench]
//!     fn fibonacci_20() {
//!         // Prevent the compiler from optimizing the call and propagating constants.
//!         std::hint::black_box(fibonacci(std::hint::black_box(20)));
//!     }
//!
//!     #[bench]
//!     fn fibonacci_45() {
//!         // Prevent the compiler from optimizing the call and propagating constants.
//!         std::hint::black_box(fibonacci(std::hint::black_box(45)));
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
//!     heap_increase: 0 pages (new)
//!     stable_memory_increase: 0 pages (new)
//!
//! ---------------------------------------------------
//!
//! Benchmark: fibonacci_45 (new)
//!   total:
//!     instructions: 3088 (new)
//!     heap_increase: 0 pages (new)
//!     stable_memory_increase: 0 pages (new)
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
//! # optionally add `--csv` to generate a CSV report
//! $ canbench --persist --csv
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
//!     heap_increase: 0 pages (no change)
//!     stable_memory_increase: 0 pages (no change)
//!
//! ---------------------------------------------------
//!
//! Benchmark: fibonacci_45
//!   total:
//!     instructions: 3088 (no change)
//!     heap_increase: 0 pages (no change)
//!     stable_memory_increase: 0 pages (no change)
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
//!     heap_increase: 0 pages (no change)
//!     stable_memory_increase: 0 pages (no change)
//!
//! ---------------------------------------------------
//!
//! Benchmark: fibonacci_45
//!   total:
//!     instructions: 56.39 B (regressed by 1826095830.76%)
//!     heap_increase: 0 pages (no change)
//!     stable_memory_increase: 0 pages (no change)
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
//! use ic_cdk::pre_upgrade;
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
//!     ic_cdk::stable::StableWriter::default()
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
//! #[cfg(feature = "canbench-rs")]
//! mod benches {
//!     use super::*;
//!     use canbench_rs::bench;
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
//! #[cfg(feature = "canbench-rs")]
//! mod benches {
//!     use super::*;
//!     use canbench_rs::bench;
//!
//!     # fn initialize_state() {}
//!     # fn pre_upgrade() {}
//!
//!     #[bench(raw)]
//!     fn pre_upgrade_bench() -> canbench_rs::BenchResult {
//!         // Some function that fills the state with lots of data.
//!         initialize_state();
//!
//!         // Only benchmark the pre_upgrade. Initializing the state isn't
//!         // included in the results of our benchmark.
//!         canbench_rs::bench_fn(pre_upgrade)
//!     }
//! }
//! ```
//!
//! Running `canbench` on the example above will benchmark only the code wrapped in `canbench_rs::bench_fn`, which in this case is the call to `pre_upgrade`.
//!
//! ```txt
//! $ canbench pre_upgrade_bench
//!
//! ---------------------------------------------------
//!
//! Benchmark: pre_upgrade_bench (new)
//!   total:
//!     instructions: 717.10 M (new)
//!     heap_increase: 519 pages (new)
//!     stable_memory_increase: 184 pages (new)
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
//! `canbench` allows you to do more granular benchmarking using the `canbench_rs::bench_scope` function.
//! Here's how we can modify our `pre_upgrade` function:
//!
//!
//! ```rust
//! # use candid::{Encode, CandidType};
//! # use ic_cdk::pre_upgrade;
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
//!         #[cfg(feature = "canbench-rs")]
//!         let _p = canbench_rs::bench_scope("serialize_state");
//!         STATE.with(|s| Encode!(s).unwrap())
//!     };
//!
//!     // Write to stable memory.
//!     #[cfg(feature = "canbench-rs")]
//!     let _p = canbench_rs::bench_scope("writing_to_stable_memory");
//!     ic_cdk::stable::StableWriter::default()
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
//!     heap_increase: 519 pages (new)
//!     stable_memory_increase: 184 pages (new)
//!
//!   serialize_state (profiling):
//!     instructions: 717.10 M (new)
//!     heap_increase: 519 pages (new)
//!     stable_memory_increase: 0 pages (new)
//!
//!   writing_to_stable_memory (profiling):
//!     instructions: 502 (new)
//!     heap_increase: 0 pages (new)
//!     stable_memory_increase: 184 pages (new)
//!
//! ---------------------------------------------------
//!
//! Executed 1 of 1 benchmarks.
//! ```
//!
//! ### Debugging
//!
//! The `ic_cdk::eprintln!()` macro facilitates tracing canister and benchmark execution.
//! Output is displayed on the console when `canbench` is executed with
//! the `--show-canister-output` option.
//!
//! ```rust
//! # #[cfg(feature = "canbench-rs")]
//! # mod benches {
//! #     use super::*;
//! #     use canbench_rs::bench;
//! #
//!     #[bench]
//!     fn bench_with_debug_print() {
//!         // Run `canbench --show-canister-output` to see the output.
//!         ic_cdk::eprintln!("Hello from {}!", env!("CARGO_PKG_NAME"));
//!     }
//! # }
//! ```
//!
//! Example output:
//!
//! ```bash
//! $ canbench bench_with_debug_print --show-canister-output
//! [...]
//! 2021-05-06 19:17:10.000000003 UTC: [Canister lxzze-o7777-77777-aaaaa-cai] Hello from example!
//! [...]
//! ```
//!
//! Refer to the [Internet Computer specification](https://internetcomputer.org/docs/references/ic-interface-spec#debugging-aids) for more details.
//!
//! ### Preventing Compiler Optimizations
//!
//! If benchmark results appear suspiciously low and remain consistent
//! despite increased benchmarked function complexity, the `std::hint::black_box`
//! function helps prevent compiler optimizations.
//!
//! ```rust
//! # #[cfg(feature = "canbench-rs")]
//! # mod benches {
//! #     use super::*;
//! #     use canbench_rs::bench;
//! #
//!     #[bench]
//!     fn fibonacci_20() {
//!         // Prevent the compiler from optimizing the call and propagating constants.
//!         std::hint::black_box(fibonacci(std::hint::black_box(20)));
//!     }
//! # }
//! ```
//!
//! Note that passing constant values as function arguments can also
//! trigger compiler optimizations. If the actual code uses
//! variables (not constants), both the arguments and the result
//! of the benchmarked function must be wrapped in `black_box` calls.
//!
//! Refer to the [Rust documentation](https://doc.rust-lang.org/std/hint/fn.black_box.html)
//! for more details.
//!
pub use canbench_rs_macros::bench;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, BTreeMap, HashMap},
    ops::AddAssign,
};

thread_local! {
    static SCOPES: RefCell<BTreeMap<&'static str, Vec<MeasurementInternal>>> =
        const { RefCell::new(BTreeMap::new()) };
}

/// The results of a benchmark.
/// This type is in a public API.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType, Default)]
pub struct BenchResult {
    /// A measurement for the entire duration of the benchmark.
    pub total: Measurement,

    /// Measurements for scopes.
    #[serde(default)]
    pub scopes: BTreeMap<String, Measurement>,
}

/// The internal representation of the benchmark result.
/// This type is not deserialized, therefore fields are not `Option`.
#[derive(Debug, PartialEq, Default)]
struct BenchResultInternal {
    /// A measurement for the entire duration of the benchmark.
    pub total: MeasurementInternal,

    /// Measurements for scopes.
    pub scopes: BTreeMap<String, MeasurementInternal>,
}

impl From<BenchResultInternal> for BenchResult {
    fn from(r: BenchResultInternal) -> Self {
        Self {
            total: Measurement::from(r.total),
            scopes: r
                .scopes
                .into_iter()
                .map(|(k, v)| (k, Measurement::from(v)))
                .collect(),
        }
    }
}

/// A benchmark measurement containing various stats.
/// This type is in a public API.
#[derive(Debug, PartialEq, Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Measurement {
    /// The number of calls made during the measurement.
    #[serde(default)]
    pub calls: u64,

    /// The number of instructions.
    #[serde(default)]
    pub instructions: u64,

    /// The increase in heap (measured in pages).
    #[serde(default)]
    pub heap_increase: u64,

    /// The increase in stable memory (measured in pages).
    #[serde(default)]
    pub stable_memory_increase: u64,
}

#[test]
fn public_api_of_measurement_should_not_change() {
    // If you have to modify this test, it's likely you broke the public API of `Measurement`.
    // Avoid making such changes unless absolutely necessary — doing so requires a major version bump.
    //
    // This test checks that the `Measurement` struct:
    // - Exists
    // - Has all expected public fields
    // - Fields have the expected names and types

    let m = Measurement {
        calls: 0_u64,
        instructions: 0_u64,
        heap_increase: 0_u64,
        stable_memory_increase: 0_u64,
    };

    // Ensure field access works and types match expectations
    let _: u64 = m.calls;
    let _: u64 = m.instructions;
    let _: u64 = m.heap_increase;
    let _: u64 = m.stable_memory_increase;
}

/// The internal representation of a measurement.
#[derive(Debug, PartialEq, Clone, Default)]
struct MeasurementInternal {
    /// Instruction counter at the start of measurement.
    /// Not in public API, because it is not supposed to be compared to other measurements.
    /// Used internally to correctly calculate instructions of overlapping or nested scopes.
    start_instructions: u64,

    /// The number of calls made during the measurement.
    pub calls: u64,

    /// The number of instructions.
    pub instructions: u64,

    /// The increase in heap (measured in pages).
    pub heap_increase: u64,

    /// The increase in stable memory (measured in pages).
    pub stable_memory_increase: u64,
}

impl From<MeasurementInternal> for Measurement {
    fn from(m: MeasurementInternal) -> Self {
        Self {
            calls: m.calls,
            instructions: m.instructions,
            heap_increase: m.heap_increase,
            stable_memory_increase: m.stable_memory_increase,
        }
    }
}

/// Benchmarks the given function.
pub fn bench_fn<R>(f: impl FnOnce() -> R) -> BenchResult {
    reset();

    let is_tracing_enabled = TRACING_BUFFER.with_borrow(|p| !p.is_empty());

    if !is_tracing_enabled {
        let start_heap = heap_size();
        let start_stable_memory = ic_cdk::api::stable_size();
        let start_instructions = instruction_count();
        f();
        let instructions = instruction_count() - start_instructions;
        let stable_memory_increase = ic_cdk::api::stable_size() - start_stable_memory;
        let heap_increase = heap_size() - start_heap;

        let total = MeasurementInternal {
            start_instructions,
            calls: 1,
            instructions,
            heap_increase,
            stable_memory_increase,
        }
        .into();
        let scopes: std::collections::BTreeMap<_, _> = get_scopes_measurements()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        BenchResult { total, scopes }
    } else {
        // The first 4 bytes are a flag to indicate if tracing is enabled. It will be read by the
        // tracing function (instrumented code) to decide whether to trace or not.
        let tracing_started_flag_address = TRACING_BUFFER.with_borrow_mut(|p| p.as_mut_ptr());
        unsafe {
            // Ideally, we'd like to reverse the following 2 statements, but it might be possible
            // for the compiler not to inline `ic_cdk::api::performance_counter` which would be
            // problematic as `performance_counter` would be traced itself. Perhaps we can call
            // ic0.performance_counter directly.
            INSTRUCTIONS_START = ic_cdk::api::performance_counter(0) as i64;
            *tracing_started_flag_address = 1;
        }
        f();
        unsafe {
            *tracing_started_flag_address = 0;
            INSTRUCTIONS_END = ic_cdk::api::performance_counter(0) as i64;
        }

        // Only the traces are meaningful, and it's written to `TRACING_BUFFER` and will be
        // collected in the tracing query method.
        BenchResult::default()
    }
}

/// Benchmarks the scope this function is declared in.
///
/// NOTE: It's important to assign this function, otherwise benchmarking won't work correctly.
///
/// # Correct Usage
///
/// ```
/// fn my_func() {
///   let _p = canbench_rs::bench_scope("my_scope");
///   // Do something.
/// }
/// ```
///
/// # Incorrect Usages
///
/// ```
/// fn my_func() {
///   let _ = canbench_rs::bench_scope("my_scope"); // Doesn't capture the scope.
///   // Do something.
/// }
/// ```
///
/// ```
/// fn my_func() {
///   canbench_rs::bench_scope("my_scope"); // Doesn't capture the scope.
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
        let start_stable_memory = ic_cdk::api::stable_size();
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
        SCOPES.with(|p| {
            let mut p = p.borrow_mut();
            let start_instructions = self.start_instructions;
            let stable_memory_increase = ic_cdk::api::stable_size() - self.start_stable_memory;
            let heap_increase = heap_size() - self.start_heap;
            let instructions = instruction_count() - self.start_instructions;
            p.entry(self.name).or_default().push(MeasurementInternal {
                start_instructions,
                calls: 1,
                instructions,
                heap_increase,
                stable_memory_increase,
            });
        });
    }
}

// Clears all scope data.
fn reset() {
    SCOPES.with(|p| p.borrow_mut().clear());
}

// Returns the measurements for any declared scopes, aggregated by the scope name.
fn get_scopes_measurements() -> BTreeMap<&'static str, Measurement> {
    fn sum_non_overlapping(measurements: &[MeasurementInternal]) -> MeasurementInternal {
        #[derive(Debug)]
        struct Interval {
            start: u64,
            end: u64,
            measurement: MeasurementInternal,
        }

        let mut intervals: Vec<Interval> = measurements
            .iter()
            .map(|m| Interval {
                start: m.start_instructions,
                end: m.start_instructions + m.instructions,
                measurement: m.clone(),
            })
            .collect();

        intervals.sort_by_key(|i| i.start);

        let mut total = MeasurementInternal::default();
        let mut current_start = 0;
        let mut current_end = 0;
        let mut group_measurements: Vec<MeasurementInternal> = Vec::new();

        for i in intervals {
            if i.start < current_end {
                current_end = current_end.max(i.end);
                group_measurements.push(i.measurement);
            } else {
                if current_end > current_start {
                    total.instructions += current_end - current_start;
                    for m in &group_measurements {
                        total.calls += m.calls;
                        total.heap_increase += m.heap_increase;
                        total.stable_memory_increase += m.stable_memory_increase;
                    }
                }
                current_start = i.start;
                current_end = i.end;
                group_measurements.clear();
                group_measurements.push(i.measurement);
            }
        }

        // Final group
        if current_end > current_start {
            total.instructions += current_end - current_start;
            for m in &group_measurements {
                total.calls += m.calls;
                total.heap_increase += m.heap_increase;
                total.stable_memory_increase += m.stable_memory_increase;
            }
        }

        total
    }

    SCOPES.with(|p| {
        p.borrow()
            .iter()
            .map(|(&scope, measurements)| {
                (scope, Measurement::from(sum_non_overlapping(measurements)))
            })
            .collect()
    })
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

thread_local! {
    static TRACING_BUFFER: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

static mut INSTRUCTIONS_START: i64 = 0;
static mut INSTRUCTIONS_END: i64 = 0;
const NUM_BYTES_ENABLED_FLAG: usize = 4;
const NUM_BYTES_NUM_ENTRIES: usize = 8;
const MAX_NUM_LOG_ENTRIES: usize = 100_000_000;
const NUM_BYTES_FUNC_ID: usize = 4;
const NUM_BYTES_INSTRUCTION_COUNTER: usize = 8;
const BUFFER_SIZE: usize = NUM_BYTES_ENABLED_FLAG
    + NUM_BYTES_NUM_ENTRIES
    + MAX_NUM_LOG_ENTRIES * (NUM_BYTES_FUNC_ID + NUM_BYTES_INSTRUCTION_COUNTER);
const LOGS_START_OFFSET: usize = NUM_BYTES_ENABLED_FLAG + NUM_BYTES_NUM_ENTRIES;
/// The maximum number of nodes in the response. This number is chosen since the
/// `InstructionTraceGraphNode` takes ~13B per node, and the maximum response size is 3MiB. This
/// number is chosen to be below 3MiB / 13B ~ 241_979.
const MAX_NUM_NODES_IN_RESPONSE: usize = 240_000;

/// A node in the instruction trace graph. The parent-child relationship is determined by the call
/// stack - if function `f` calls function `g`, then `g` is a child of `f`.
#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct InstructionTraceGraphNode {
    /// The id of the function.
    pub func_id: i32,
    /// The instruction cost of the function, not including its descendants.
    pub cost: i64,
    /// The children of the graph node.
    pub children: Vec<InstructionTraceGraphNode>,
}

#[export_name = "__prepare_tracing"]
fn prepare_tracing() -> i32 {
    TRACING_BUFFER.with_borrow_mut(|b| {
        *b = vec![0; BUFFER_SIZE];
        b.as_ptr() as i32
    })
}

pub fn get_traces(
    bench_instructions: u64,
    aggregate: bool,
) -> Result<InstructionTraceGraphNode, String> {
    TRACING_BUFFER.with_borrow(|b| {
        if b[0] == 1 {
            panic!("Tracing is still enabled.");
        }
        let num_entries = i64::from_le_bytes(
            b[NUM_BYTES_ENABLED_FLAG..(NUM_BYTES_ENABLED_FLAG + NUM_BYTES_NUM_ENTRIES)]
                .try_into()
                .unwrap(),
        );
        if num_entries > MAX_NUM_LOG_ENTRIES as i64 {
            return Err(format!(
                "There are {num_entries} log entries which is more than \
                {MAX_NUM_LOG_ENTRIES}, as we can currently support",
            ));
        }
        let instructions_start = unsafe { INSTRUCTIONS_START };
        let mut traces = vec![(i32::MAX, 0)];
        for i in 0..num_entries {
            let log_start_address = i as usize
                * (NUM_BYTES_FUNC_ID + NUM_BYTES_INSTRUCTION_COUNTER)
                + LOGS_START_OFFSET;
            let func_id = i32::from_le_bytes(
                b[log_start_address..log_start_address + NUM_BYTES_FUNC_ID]
                    .try_into()
                    .unwrap(),
            );
            let instruction_counter = i64::from_le_bytes(
                b[log_start_address + NUM_BYTES_FUNC_ID
                    ..log_start_address + NUM_BYTES_FUNC_ID + NUM_BYTES_INSTRUCTION_COUNTER]
                    .try_into()
                    .unwrap(),
            );
            traces.push((func_id, instruction_counter - instructions_start));
        }
        traces.push((i32::MIN, unsafe { INSTRUCTIONS_END - instructions_start }));
        let traces = adjust_traces_for_overhead(traces, bench_instructions);
        // TODO(EXC-2020): consider using compression.
        let graph = convert_traces_to_graph(traces);
        let graph = if aggregate {
            agggregate_traces(graph)
        } else {
            graph
        };
        let graph = truncate_traces_graph(graph);
        Ok(graph)
    })
}

/// Converts traces from a list of trace events (start or end of a function and the instruction
/// count at that point) to a graph, where the parent-child relationship is determined by the call
/// stack.
fn convert_traces_to_graph(traces: Vec<(i32, i64)>) -> InstructionTraceGraphNode {
    let (node, _, _) = convert_traces_range_to_graph(&traces, 0);
    node
}

/// Converts `traces[start..]` to a graph. The return value is `(node, next_start, total_cost)`,
/// where `node` is the graph node which corresponds to the function at `traces[start]`,
/// `next_start` is the index of the next trace to process, and `total_cost` is the total cost of
/// the children of the current function. The additional 2 values are used for recursion.
fn convert_traces_range_to_graph(
    traces: &[(i32, i64)],
    start: usize,
) -> (InstructionTraceGraphNode, usize, i64) {
    let (func_id, start_counter) = traces[start];
    assert!(func_id >= 0, "Traces are corrupted {}.", func_id);

    // The following loop tries to find all children of the current function.
    let mut next_start = start + 1;
    let mut children = Vec::new();
    let mut total_children_cost = 0;
    loop {
        // If the next trace is an end of a function, we've found all children. The function id
        // should be `!func_id``, and it will be asserted after exiting the loop.
        if traces[next_start].0 < 0 {
            break;
        }
        let (child, new_next_start, child_cost) = convert_traces_range_to_graph(traces, next_start);
        children.push(child);
        next_start = new_next_start;
        total_children_cost += child_cost;
    }

    let (end_func_id, end_counter) = traces[next_start];
    assert_eq!(func_id, !end_func_id, "Traces are corrupted.");
    let total_cost = end_counter - start_counter;
    let cost = total_cost - total_children_cost;
    let node = InstructionTraceGraphNode {
        func_id,
        cost,
        children,
    };
    (node, next_start + 1, total_cost)
}

fn adjust_traces_for_overhead(traces: Vec<(i32, i64)>, bench_instructions: u64) -> Vec<(i32, i64)> {
    let num_logs = traces.len() - 2;
    let overhead = (traces[num_logs].1 as f64 - bench_instructions as f64) / (num_logs as f64);
    traces
        .into_iter()
        .enumerate()
        .map(|(i, (id, count))| {
            if i <= num_logs {
                (id, count - (overhead * i as f64) as i64)
            } else {
                (id, count - (overhead * num_logs as f64) as i64)
            }
        })
        .collect()
}

/// Truncates the traces graph by depth, and depth (above which the nodes get truncated) is the
/// largest one such that the total number of nodes is less than or equal to
/// `MAX_NUM_NODES_IN_RESPONSE`.
fn truncate_traces_graph(root: InstructionTraceGraphNode) -> InstructionTraceGraphNode {
    let mut num_nodes_by_depth = BTreeMap::new();
    increase_num_nodes_by_depth(&root, 0, &mut num_nodes_by_depth);

    let mut depth_to_truncate = 0;
    let mut cumulative_nodes = 0;
    for (depth, num_nodes) in num_nodes_by_depth.iter() {
        cumulative_nodes += num_nodes;
        if cumulative_nodes <= MAX_NUM_NODES_IN_RESPONSE {
            depth_to_truncate = *depth;
        } else {
            break;
        }
    }

    truncate_traces_graph_by_depth(root, depth_to_truncate)
}

/// A recursive function used to count the number of nodes in the graph by depth.
fn increase_num_nodes_by_depth(
    node: &InstructionTraceGraphNode,
    depth: usize,
    num_nodes_by_depth: &mut BTreeMap<usize, usize>,
) {
    *num_nodes_by_depth.entry(depth).or_insert(0) += 1;
    for child in &node.children {
        increase_num_nodes_by_depth(child, depth + 1, num_nodes_by_depth);
    }
}

/// A recursive function to truncates the graph by the given depth. When the depth is 0, it means
/// only the node itself should be kept and its children should be truncated.
fn truncate_traces_graph_by_depth(
    node: InstructionTraceGraphNode,
    depth: usize,
) -> InstructionTraceGraphNode {
    let InstructionTraceGraphNode {
        func_id,
        cost,
        children,
    } = node;
    let children = if depth == 0 {
        Vec::new()
    } else {
        children
            .into_iter()
            .map(|child| truncate_traces_graph_by_depth(child, depth - 1))
            .collect()
    };
    InstructionTraceGraphNode {
        func_id,
        cost,
        children,
    }
}

/// An auxiliary struct used to aggregate the traces. Compared to `InstructionTraceGraphNode`, it
/// has an additional field `total_cost`, which is the total cost of the node, including its
/// descendants, so that the children can be sorted by the total cost.
struct AggregatedInstructionTraceGraphNode {
    func_id: i32,
    /// The cost of the node, not including its descendants.
    cost: i64,
    /// The total cost of the node, including its descendants.
    total_cost: i64,
    /// The children of the node. The key is the function id, and the value is the aggregated child.
    children: HashMap<i32, AggregatedInstructionTraceGraphNode>,
}

impl AggregatedInstructionTraceGraphNode {
    /// Aggregates the traces of a function into the aggregated node.
    fn aggregate(node: InstructionTraceGraphNode) -> Self {
        let InstructionTraceGraphNode {
            func_id,
            cost,
            children,
        } = node;
        let mut aggregated_children = HashMap::new();
        let mut total_cost = cost;
        for child in children {
            let aggregated_child = Self::aggregate(child);
            total_cost += aggregated_child.total_cost;
            match aggregated_children.entry(aggregated_child.func_id) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += aggregated_child;
                }
                Entry::Vacant(entry) => {
                    entry.insert(aggregated_child);
                }
            }
        }
        Self {
            func_id,
            cost,
            total_cost,
            children: aggregated_children,
        }
    }
}

impl AddAssign for AggregatedInstructionTraceGraphNode {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(
            self.func_id, other.func_id,
            "Cannot aggregate traces with different func_ids."
        );
        self.cost += other.cost;
        self.total_cost += other.total_cost;
        for (other_func_id, other_child) in other.children {
            match self.children.entry(other_func_id) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += other_child;
                }
                Entry::Vacant(entry) => {
                    entry.insert(other_child);
                }
            }
        }
    }
}

impl From<AggregatedInstructionTraceGraphNode> for InstructionTraceGraphNode {
    /// Converts the aggregated node to the original node.
    fn from(node: AggregatedInstructionTraceGraphNode) -> Self {
        let AggregatedInstructionTraceGraphNode {
            func_id,
            cost,
            total_cost: _,
            children,
        } = node;
        let mut children: Vec<_> = children.into_iter().map(|(_, child)| child).collect();
        children.sort_by_key(|child| -child.total_cost);
        let children = children
            .into_iter()
            .map(InstructionTraceGraphNode::from)
            .collect();
        InstructionTraceGraphNode {
            func_id,
            cost,
            children,
        }
    }
}

/// Converts the traces graph from time-based to aggregated (children sorted by total cost).
fn agggregate_traces(root: InstructionTraceGraphNode) -> InstructionTraceGraphNode {
    let aggregated = AggregatedInstructionTraceGraphNode::aggregate(root);
    InstructionTraceGraphNode::from(aggregated)
}
