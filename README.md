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

| :memo:        | `canbench` currently supports Rust canisters, but support for more languages can be introduced in the future. |
|---------------|:------------------------|

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
Replace the `fibonacci` method defined previously with the following:

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

Apparently, the recursive implementation is many orders of magniture more expensive than the iterative implementation 😱
Good thing we found out before deploying this implementation to production.

| :memo:        | Notice that the `fibonacci_45` took > 50B instructions, which is substantially more than the instruction limit given for a single message execution on the Internet Computer. `canbench` runs benchmarks in an environment that gives them up to 10T instructions |
|---------------|:------------------------|
