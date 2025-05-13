use canbench_rs::{BenchResult, Measurement};
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
            scope: scope.map(str::to_string),
        }
    }

    pub(crate) fn full_name(&self) -> String {
        match &self.scope {
            Some(scope) => format!("{}::{}", self.name, scope),
            None => self.name.clone(),
        }
    }
}

pub(crate) struct Data {
    new: Option<u64>,
    old: Option<u64>,
}

impl Data {
    pub(crate) fn value(&self) -> Option<u64> {
        self.new
    }

    pub(crate) fn abs_delta(&self) -> Option<i64> {
        match (self.new, self.old) {
            (Some(new), Some(old)) => Some(new as i64 - old as i64),
            _ => None,
        }
    }

    pub(crate) fn percent_diff(&self) -> Option<f64> {
        match (self.new, self.old) {
            (Some(new), Some(old)) => {
                if old == 0 {
                    match new {
                        0 => Some(0.0),
                        _ if new > 0 => Some(f64::INFINITY),
                        _ => Some(f64::NEG_INFINITY),
                    }
                } else {
                    Some((new as f64 - old as f64) / old as f64 * 100.0)
                }
            }
            _ => None,
        }
    }
}

pub(crate) fn extract(
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
) -> Vec<Entry> {
    let mut results = Vec::new();
    let mut processed = BTreeSet::new();

    let mut push_entry = |status: &str,
                          benchmark: Benchmark,
                          new_m: Option<&Measurement>,
                          old_m: Option<&Measurement>| {
        let make_data = |f: fn(&Measurement) -> u64| Data {
            new: new_m.map(f),
            old: old_m.map(f),
        };
        results.push(Entry {
            status: status.to_string(),
            benchmark,
            instructions: make_data(|m| m.instructions),
            heap_increase: make_data(|m| m.heap_increase),
            stable_memory_increase: make_data(|m| m.stable_memory_increase),
        });
    };

    for (new_name, new_bench) in new_results {
        let old_bench = old_results.get(new_name);
        let status = if old_bench.is_some() { "" } else { "new" };
        let benchmark = Benchmark::new(new_name, None);
        processed.insert(benchmark.clone());
        push_entry(
            status,
            benchmark,
            Some(&new_bench.total),
            old_bench.map(|b| &b.total),
        );

        for (scope, new_m) in &new_bench.scopes {
            let old_m = old_bench.and_then(|b| b.scopes.get(scope));
            let status = if old_m.is_some() { "" } else { "new" };
            let benchmark = Benchmark::new(new_name, Some(scope));
            processed.insert(benchmark.clone());
            push_entry(status, benchmark, Some(new_m), old_m);
        }
    }

    for (old_name, old_bench) in old_results {
        let benchmark = Benchmark::new(old_name, None);
        if !processed.contains(&benchmark) {
            push_entry("removed", benchmark, None, Some(&old_bench.total));
        }

        for (scope, old_m) in &old_bench.scopes {
            let benchmark = Benchmark::new(old_name, Some(scope));
            if !processed.contains(&benchmark) {
                push_entry("removed", benchmark, None, Some(old_m));
            }
        }
    }

    results
}
