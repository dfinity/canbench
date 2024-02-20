<p>
  <a href="https://github.com/dfinity/canbench/blob/main/LICENSE"><img alt="Apache-2.0" src="https://img.shields.io/github/license/dfinity/bench"/></a>
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
`canbench` provides developers the tools and insights to understand how their code is using instructions and memory.
Support for reporting dirty pages will be available once there's a way to retrieve dirty page information from the IC.

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

## Quickstart (Rust)

See the [crate's documentation](https://docs.rs/canbench-rs).

## Github CI Support

`canbench` can be included in Github CI to automatically detect performance changes.
Have a look at the workflows in this repository for working examples.
A github CI action looks like the following.
Note you'll need to copy the scripts in the `scripts` directory to your own repository and update `<PATH/TO/YOUR/CANISTER>`.

```yaml
  benchmark-my-canister:
    runs-on: ubuntu-latest
    env:
      PROJECT_DIR: <PATH/TO/YOUR/CANISTER>
    steps:
      - name: Checkout current PR
        uses: actions/checkout@v4

      - name: Checkout main branch
        uses: actions/checkout@v4
        with:
          ref: main
          path: _canbench_main_branch

      - name: Install Rust
        run: |
          rustup update $RUST_VERSION --no-self-update
          rustup default $RUST_VERSION
          rustup target add wasm32-unknown-unknown

      - name: Benchmark
        run: |
          bash ./scripts/ci_run_benchmark.sh $PROJECT_DIR

      - name: Post comment
        uses: thollander/actions-comment-pull-request@v2
        with:
          filePath: /tmp/canbench_comment_message.txt
          comment_tag: ${{ env.PROJECT_DIR }}

      - name: Pass or fail
        run: |
          bash ./scripts/ci_post_run_benchmark.sh
```

Once you have the CI job above set up, the job will pass if there are no significant performance changes detected and fail otherwise.
A comment is added to the PR to show the results. See [this PR](https://github.com/dfinity/bench/pull/18) for an example.
