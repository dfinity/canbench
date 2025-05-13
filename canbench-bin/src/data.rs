use crate::fmt::fmt_human_percent;
use canbench_rs::{BenchResult, Measurement};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Entry {
    pub(crate) status: String,
    pub(crate) benchmark: Benchmark,
    pub(crate) instructions: Values,
    pub(crate) heap_increase: Values,
    pub(crate) stable_memory_increase: Values,
}

impl Entry {
    pub(crate) fn has_scope(&self) -> bool {
        self.benchmark.scope.is_some()
    }
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
        self.scope
            .as_ref()
            .map(|s| format!("{}::{}", self.name, s))
            .unwrap_or_else(|| self.name.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Change {
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
        Some(self.new? as i64 - self.old? as i64)
    }

    pub(crate) fn percent_diff(&self) -> Option<f64> {
        let delta = self.abs_delta()?;
        let old = self.old?;

        Some(if old == 0 {
            match delta {
                d if d < 0 => f64::NEG_INFINITY,
                d if d > 0 => f64::INFINITY,
                _ => 0.0,
            }
        } else {
            delta as f64 / old as f64 * 100.0
        })
    }

    pub(crate) fn fmt_current(&self) -> String {
        self.current().map_or_else(String::new, |v| v.to_string())
    }

    pub(crate) fn fmt_human_percent(&self) -> String {
        self.percent_diff()
            .map_or_else(String::new, fmt_human_percent)
    }

    pub(crate) fn status(&self, noise_threshold: f64) -> Change {
        match (self.new, self.old) {
            (Some(_), Some(_)) => match self.percent_diff() {
                Some(p) if p.abs() < noise_threshold => Change::Unchanged,
                Some(p) if p < 0.0 => Change::Improved,
                Some(_) => Change::Regressed,
                _ => unreachable!(),
            },
            (Some(_), None) => Change::New,
            _ => unreachable!(),
        }
    }
}

pub(crate) fn extract(
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
) -> Vec<Entry> {
    let mut results = Vec::new();

    for (name, new_bench) in new_results {
        let old_bench = old_results.get(name);

        // Process total
        results.push(build_entry(
            old_bench.is_some(),
            Benchmark::new(name, None),
            Some(&new_bench.total),
            old_bench.map(|b| &b.total),
        ));

        // Process scopes
        for (scope, new_m) in &new_bench.scopes {
            let old_m = old_bench.and_then(|b| b.scopes.get(scope));

            results.push(build_entry(
                old_m.is_some(),
                Benchmark::new(name, Some(scope)),
                Some(new_m),
                old_m,
            ));
        }
    }

    results
}

fn build_entry(
    old_present: bool,
    benchmark: Benchmark,
    new_m: Option<&Measurement>,
    old_m: Option<&Measurement>,
) -> Entry {
    let extract_values = |f: fn(&Measurement) -> u64| Values {
        new: new_m.map(f),
        old: old_m.map(f),
    };

    Entry {
        status: if old_present {
            "".to_string()
        } else {
            "new".to_string()
        },
        benchmark,
        instructions: extract_values(|m| m.instructions),
        heap_increase: extract_values(|m| m.heap_increase),
        stable_memory_increase: extract_values(|m| m.stable_memory_increase),
    }
}
