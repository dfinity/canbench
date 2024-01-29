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
