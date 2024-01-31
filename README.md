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
