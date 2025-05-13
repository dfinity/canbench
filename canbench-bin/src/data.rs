use canbench_rs::{BenchResult, Measurement};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Entry {
    status: String,
    benchmark: Benchmark,
    pub(crate) instructions: Values,
    pub(crate) heap_increase: Values,
    pub(crate) stable_memory_increase: Values,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Status {
    Unchanged,
    New,
    Improved,
    Regressed,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Values {
    new: Option<u64>,
    old: Option<u64>,
}

impl Values {
    pub(crate) fn current(&self) -> Option<u64> {
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

    pub(crate) fn status(&self, noise_threshold: f64) -> Status {
        match (self.new, self.old) {
            (Some(new), Some(old)) => {
                let abs_delta = new as i64 - old as i64;
                if old == 0 {
                    match abs_delta {
                        d if d < 0 => Status::Improved,
                        d if d > 0 => Status::Regressed,
                        _ => Status::Unchanged,
                    }
                } else {
                    let delta = abs_delta as f64 / old as f64 * 100.0;
                    if delta.abs() < noise_threshold {
                        Status::Unchanged
                    } else if delta < 0.0 {
                        Status::Improved
                    } else {
                        Status::Regressed
                    }
                }
            }
            (Some(_), None) => Status::New,
            _ => unreachable!(),
        }
    }
}

pub(crate) fn extract(
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
) -> Vec<Entry> {
    let mut results = Vec::new();

    let mut push_entry = |status: &str,
                          benchmark: Benchmark,
                          new_m: Option<&Measurement>,
                          old_m: Option<&Measurement>| {
        let to_values = |f: fn(&Measurement) -> u64| Values {
            new: new_m.map(f),
            old: old_m.map(f),
        };
        results.push(Entry {
            status: status.to_string(),
            benchmark,
            instructions: to_values(|m| m.instructions),
            heap_increase: to_values(|m| m.heap_increase),
            stable_memory_increase: to_values(|m| m.stable_memory_increase),
        });
    };

    for (new_name, new_bench) in new_results {
        let old_bench = old_results.get(new_name);
        let status = if old_bench.is_some() { "" } else { "new" };
        let benchmark = Benchmark::new(new_name, None);
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
            push_entry(status, benchmark, Some(new_m), old_m);
        }
    }

    results
}
