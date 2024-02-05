<p>
  <a href="https://github.com/dfinity/stable-structures/blob/master/LICENSE"><img alt="Apache-2.0" src="https://img.shields.io/github/license/dfinity/bench"/></a>
  <a href="https://forum.dfinity.org/"><img alt="Chat on the Forum" src="https://img.shields.io/badge/help-post%20on%20forum.dfinity.org-blue"></a>
</p>

# `canbench`

`canbench` is a tool for benchmarking canisters on the Internet Computer.

## Background

Canister smart contracts on the Internet Computer consume compute and memory resources.
Given that resources are finite, there are bounds in place when canisters execute a message (transaction):

1. __Instructions__: a monotonically increasing counter that's corelated with the amount of computation and memory accesses.
2. __Dirty Pages__: the number of memory pages that are written to.

A single message execution must stay within the allowed bounds, otherwise it's terminated.
`canbench` provides developers the tools and insights to understand how their code is using these resources.

## Use Cases

* Understanding how a canister consumes instructions, heap memory, and stable memory.
* Detecting performance regressions locally or on CI.
* Analyzing where performance bottlenecks are present.

## Features

* __Metrics that are relevant__

  Typically benchmarking tools run a benchmark multiple times and return the average time.
  On the Internet Computer, where instrumentation is deterministic, this approach is neither ideal nor insightful.
  Instead, `canbench` reports the number of instructions consumed, as well as changes to both heap and stable memories.

* __Easy detection of regressions__

  `canbench` allows you to persist the benchmarking results in your canister's repository.
  Storing the benchmarking results allows `canbench` to determine how the performance has changed relative to the past to detect regressions.

* __Generous instruction limit__

  While messages on the Internet Computer are bound to a few billion instructions, `canbench` can run benchmarks that are up to 10 trillion instructions, giving you the freedom to write resource-intensive benchmarks as needed.

* __Language Agnostic__

  `canbench` can, in theory, benchmark canisters written in any language. Initially support for only Rust exists, but support for additional languages can easily be introduced.

## Installation

```bash
cargo install canbench
```

## Quickstart

| :memo: NOTE          |
|:---------------------------|
| This example is also available to tinker with in the examples directory. See the [fibonacci example](./examples/fibonacci). |

### 1. Add optional dependency to `Cargo.toml`

Typically you do not want your benchmarks to be part of your canister when deploying it to the Internet Computer.
Therefore, we include `canbench` only as an optional dependency so that it's only included when running benchmarks.
For more information about optional dependencies, you can read more about them [here](https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies).

```toml
canbench = { version = "x.y.z", optional = true }
```

### 2. Add a configuration to `canbench.yml`

The `canbench.yml` configuration file tells `canbench` how to build and run you canister.
Below is a typical configuration.
Note that we're compiling the canister with the `canbench` feature so that the benchmarking logic is included in the Wasm.

```yml
build_cmd:
  cargo build --release --target wasm32-unknown-unknown --features canbench

wasm_path:
  ./target/wasm32-unknown-unknown/release/<YOUR_CANISTER>.wasm
```

### 3. Start benching! 🏋🏽

Let's say we have a canister that exposes a `query` computing the fibonacci sequence of a given number.
Here's what that query can look like:

```rust
#[ic_cdk::query]
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    let mut result = 0;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    result
}
```

Now, let's add some benchmarks to this query:

```rust
#[cfg(feature = "canbench")]
mod benches {
    use super::*;
    use canbench::bench;

    #[bench]
    fn fibonacci_20() {
        // NOTE: the result is printed to prevent the compiler from optimizing the call away.
        println!("{:?}", fibonacci(20));
    }

    #[bench]
    fn fibonacci_45() {
        // NOTE: the result is printed to prevent the compiler from optimizing the call away.
        println!("{:?}", fibonacci(45));
    }
}
```

Run `canbench`. You'll see an output that looks similar to this:

```txt
$ canbench

---------------------------------------------------

Benchmark: fibonacci_20 (new)
  total:
    instructions: 2301 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------

Benchmark: fibonacci_45 (new)
  total:
    instructions: 3088 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 0 pages (new)

---------------------------------------------------

Executed 2 of 2 benchmarks.
```

### 4. Track performance regressions

Notice that `canbench` reported the above benchmarks as "new".
`canbench` allows you to persist the results of these benchmarks.
In subsequent runs, `canbench` reports the performance relative to the last persisted run.

Let's first persist the results above by running `canbench` again, but with the `persist` flag:

```txt
$ canbench --persist
...
---------------------------------------------------

Executed 2 of 2 benchmarks.
Successfully persisted results to canbench_results.yml
```

Now, if we run `canbench` again, `canbench` will run the benchmarks, and will additionally report that there were no changes detected in performance.

```txt
$ canbench
    Finished release [optimized] target(s) in 0.34s

---------------------------------------------------

Benchmark: fibonacci_20
  total:
    instructions: 2301 (no change)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

---------------------------------------------------

Benchmark: fibonacci_45
  total:
    instructions: 3088 (no change)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

---------------------------------------------------

Executed 2 of 2 benchmarks.
```

Let's try swapping out our implementation of `fibonacci` with an implementation that's miserably inefficient.
Replace the `fibonacci` function defined previously with the following:

```rust
#[ic_cdk::query]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

And running `canbench` again, we see that it detects and reports a regression.

```txt
$ canbench

---------------------------------------------------

Benchmark: fibonacci_20
  total:
    instructions: 337.93 K (regressed by 14586.14%)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

---------------------------------------------------

Benchmark: fibonacci_45
  total:
    instructions: 56.39 B (regressed by 1826095830.76%)
    heap_delta: 0 pages (no change)
    stable_memory_delta: 0 pages (no change)

---------------------------------------------------

Executed 2 of 2 benchmarks.
```

Apparently, the recursive implementation is many orders of magnitude more expensive than the iterative implementation 😱
Good thing we found out before deploying this implementation to production.

| :memo: NOTE          |
|:---------------------------|
| Notice that `fibonacci_45` took > 50B instructions, which is substantially more than the instruction limit given for a single message execution on the Internet Computer. `canbench` runs benchmarks in an environment that gives them up to 10T instructions |

## Additional Examples

For the following examples, we'll be using the following canister code, which you can also find in the [examples](./examples/btreemap_vs_hashmap) directory.
This canister defines a simple state as well as a `pre_upgrade` function that stores that state into stable memory.

```rust
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
```

### Excluding setup code

Let's say we want to benchmark how long it takes to run the `pre_upgrade` function. We can define the following benchmark:

```rust
#[cfg(feature = "canbench")]
mod benches {
    use super::*;
    use canbench::bench;

    #[bench]
    fn pre_upgrade_bench() {
        // Some function that fills the state with lots of data.
        initialize_state();

        pre_upgrade();
    }
}
```

The problem with the above benchmark is that it's benchmarking both the `pre_upgrade` call _and_ the initialization of the state.
What if we're only interested in benchmarking the `pre_upgrade` call?
To address this, we can use the `#[bench(raw)]` macro to specify exactly which code we'd like to benchmark.

```rust
#[cfg(feature = "canbench")]
mod benches {
    use super::*;
    use canbench::bench;

    #[bench(raw)]
    fn pre_upgrade_bench() -> canbench::BenchResult {
        // Some function that fills the state with lots of data.
        initialize_state();

        // Only benchmark the pre_upgrade. Initializing the state isn't
        // included in the results of our benchmark.
        canbench::benchmark(pre_upgrade)
    }
}
```

Running `canbench` on the example above will benchmark only the code wrapped in `canbench::benchmark`, which in this case is the call to `pre_upgrade`.

```txt
$ canbench pre_upgrade_bench

---------------------------------------------------

Benchmark: pre_upgrade_bench (new)
  total:
    instructions: 717.10 M (new)
    heap_delta: 519 pages (new)
    stable_memory_delta: 184 pages (new)

---------------------------------------------------

Executed 1 of 1 benchmarks.
```

### Granular Benchmarking

Building on the example above, the `pre_upgrade` function does two steps:

1. Serialize the state
2. Write to stable memory

Suppose we're interested in understanding, within `pre_upgrade`, the resources spent in each of these steps.
`canbench` allows you to do more granular benchmarking using the `canbench::profile` function.
Here's how we can modify our `pre_upgrade` function:


```rust
#[pre_upgrade]
fn pre_upgrade() {
    // Serialize state.
    let bytes = {
        #[cfg(feature = "canbench")]
        let _p = canbench::profile("serialize_state");
        STATE.with(|s| Encode!(s).unwrap())
    };

    // Write to stable memory.
    #[cfg(feature = "canbench")]
    let _p = canbench::profile("writing_to_stable_memory");
    ic_cdk::api::stable::StableWriter::default()
        .write(&bytes)
        .unwrap();
}
```

In the code above, we've asked `canbench` to profile each of these steps separately.
Running `canbench` now, each of these steps are reported.

```txt
$ canbench pre_upgrade_bench

---------------------------------------------------

Benchmark: pre_upgrade_bench (new)
  total:
    instructions: 717.11 M (new)
    heap_delta: 519 pages (new)
    stable_memory_delta: 184 pages (new)

  serialize_state (profiling):
    instructions: 717.10 M (new)
    heap_delta: 519 pages (new)
    stable_memory_delta: 0 pages (new)

  writing_to_stable_memory (profiling):
    instructions: 502 (new)
    heap_delta: 0 pages (new)
    stable_memory_delta: 184 pages (new)

---------------------------------------------------

Executed 1 of 1 benchmarks.
```

### Github CI Support

`canbench` can be included in Github CI to automatically detect performance changes.
Have a look at the workflows in this repository for working examples.
A github CI action looks like the following.
Note you'll need to copy the scripts in the `scripts` directory to your own repository and update `<PATH/TO/YOUR/CANISTER>`.

```
  benchmark-my-canister:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}-1

      - name: Install Rust
        run: |
          rustup update ${{ matrix.rust }} --no-self-update
          rustup default ${{ matrix.rust }}
          rustup target add wasm32-unknown-unknown

      - name: Benchmark
        run: |
          bash ./scripts/ci_run_benchmark.sh <PATH/TO/YOUR/CANISTER>

      - name: Post comment
        uses: thollander/actions-comment-pull-request@v2
        with:
          filePath: /tmp/canbench_comment_message.txt
          comment_tag: canbench    <-- make sure this tag is unique if you're benchmarking multiple canisters.

      - name: Pass or fail
        run: |
          bash ./scripts/ci_post_run_benchmark.sh
```

Once you have the CI job above set up, the job will pass if there are no significant performance changes detected and fail otherwise.
A comment is added to the PR to show the results. See [this PR](https://github.com/dfinity/bench/pull/18) for an example.
