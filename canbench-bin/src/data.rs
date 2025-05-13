use canbench_rs::{BenchResult, Measurement};
use candid::de;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct Entry {
    status: String,
    benchmark: Benchmark,
    instructions: Data,
    heap_increase: Data,
    stable_memory_increase: Data,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Benchmark {
    name: String,
    scope: Option<String>,
}

impl Benchmark {
    pub(crate) fn new(name: &str, scope: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            scope: scope.map(|s| s.to_string()),
        }
    }
}

pub(crate) struct Data {
    new: Option<u64>,
    old: Option<u64>,
}

pub(crate) fn extract(
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
) -> Vec<Entry> {
    let mut results = Vec::new();
    let mut processed = BTreeSet::new();

    // Process new or modified benchmarks.
    for (new_name, new_bench) in new_results {
        // Process total measurements.
        let old_bench = old_results.get(new_name);
        let old = old_bench.map(|b| &b.total);
        let status = if old.is_some() { "" } else { "new" }.to_string();
        let benchmark = Benchmark::new(new_name, None);
        processed.insert(benchmark.clone());
        let make_data = |extract: fn(&Measurement) -> u64| Data {
            new: Some(extract(&new_bench.total)),
            old: old.map(extract),
        };
        results.push(Entry {
            status,
            benchmark,
            instructions: make_data(|m| m.instructions),
            heap_increase: make_data(|m| m.heap_increase),
            stable_memory_increase: make_data(|m| m.stable_memory_increase),
        });

        // Process scope measurements.
        for (scope, new) in new_bench.scopes.iter() {
            let old = old_bench.and_then(|b| b.scopes.get(scope));
            let status = if old.is_some() { "" } else { "new" }.to_string();
            let benchmark = Benchmark::new(new_name, Some(scope));
            processed.insert(benchmark.clone());
            let make_data = |extract: fn(&Measurement) -> u64| Data {
                new: Some(extract(new)),
                old: old.map(extract),
            };
            results.push(Entry {
                status,
                benchmark,
                instructions: make_data(|m| m.instructions),
                heap_increase: make_data(|m| m.heap_increase),
                stable_memory_increase: make_data(|m| m.stable_memory_increase),
            });
        }
    }

    // Process removed benchmarks.
    for (old_name, old_bench) in old_results {
        let benchmark = Benchmark::new(old_name, None);
        if !processed.contains(&benchmark) {
            let make_data = |extract: fn(&Measurement) -> u64| Data {
                new: None,
                old: Some(extract(&old_bench.total)),
            };
            results.push(Entry {
                status: "removed".to_string(),
                benchmark,
                instructions: make_data(|m| m.instructions),
                heap_increase: make_data(|m| m.heap_increase),
                stable_memory_increase: make_data(|m| m.stable_memory_increase),
            });
        }
        for (scope, old) in old_bench.scopes.iter() {
            let benchmark = Benchmark::new(old_name, Some(scope));
            if !processed.contains(&benchmark) {
                let make_data = |extract: fn(&Measurement) -> u64| Data {
                    new: None,
                    old: Some(extract(old)),
                };
                results.push(Entry {
                    status: "removed".to_string(),
                    benchmark,
                    instructions: make_data(|m| m.instructions),
                    heap_increase: make_data(|m| m.heap_increase),
                    stable_memory_increase: make_data(|m| m.stable_memory_increase),
                });
            }
        }
    }

    results
}
