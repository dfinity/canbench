use canbench_rs::{BenchResult, Measurement};
use candid::de;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct Entry {
    status: String,
    benchmark: Benchmark,
    // instructions: Data,
    // heap_increase: Data,
    // stable_memory_increase: Data,
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

// pub(crate) struct Data {
//     value: f64,
// }

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
        results.push(Entry { status, benchmark });

        // Process scope measurements.
        for (scope, _new) in new_bench.scopes.iter() {
            let old = old_bench.and_then(|b| b.scopes.get(scope));
            let status = if old.is_some() { "" } else { "new" }.to_string();
            let benchmark = Benchmark::new(new_name, Some(scope));
            processed.insert(benchmark.clone());
            results.push(Entry { status, benchmark });
        }
    }

    // Process removed benchmarks.
    for (old_name, old_bench) in old_results {
        let old = Benchmark::new(old_name, None);
        if !processed.contains(&old) {
            results.push(Entry {
                status: "removed".to_string(),
                benchmark: old,
            });
        }
        for (scope, _old) in old_bench.scopes.iter() {
            let old = Benchmark::new(old_name, Some(scope));
            if !processed.contains(&old) {
                results.push(Entry {
                    status: "removed".to_string(),
                    benchmark: old,
                });
            }
        }
    }

    results
}
