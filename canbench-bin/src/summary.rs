use crate::data::{Entry, Status, Values};
use canbench_rs::{BenchResult, Measurement};
use std::{collections::BTreeMap, f64};

pub(crate) fn print_summary(data: &Vec<Entry>, noise_threshold: f64) {
    println!("Summary:");
    print_metric_summary("instructions", data, noise_threshold, |e| &e.instructions);
    println!();
    print_metric_summary("heap_increase", data, noise_threshold, |e| &e.heap_increase);
    println!();
    print_metric_summary("stable_memory_increase", data, noise_threshold, |e| {
        &e.stable_memory_increase
    });
}

fn print_metric_summary<F>(label: &str, data: &Vec<Entry>, noise_threshold: f64, extractor: F)
where
    F: Fn(&Entry) -> &Values,
{
    let mut new = 0;
    let mut improved = 0;
    let mut regressed = 0;
    let mut unchanged = 0;

    let mut abs_deltas = Vec::new();
    let mut percent_diffs = Vec::new();

    for entry in data {
        // In summary only show the total measurements.
        if entry.has_scope() {
            continue;
        }

        let values = extractor(entry);
        if let Some(delta) = values.abs_delta() {
            abs_deltas.push(delta);
        }
        if let Some(percent) = values.percent_diff() {
            percent_diffs.push(percent);
        }
        match values.status(noise_threshold) {
            Status::New => {
                new += 1;
            }
            Status::Improved => {
                improved += 1;
            }
            Status::Regressed => {
                regressed += 1;
            }
            Status::Unchanged => {
                unchanged += 1;
            }
        }
    }

    let total = new + improved + regressed + unchanged;
    debug_assert_eq!(total, data.len(), "total count mismatch");

    println!("  {label}:");
    let status = match (improved, regressed) {
        (0, 0) => "No significant changes detected 👍",
        (_, 0) => "Improvements detected! 🟢",
        (0, _) => "Regressions detected! 🔴",
        _ => "Both improvements and regressions detected! 🟢🔴",
    };
    println!("    status:   {status}");
    println!(
        "    counts:   [total {} | new {} | improved {} | regressed {} | unchanged {}]",
        total, new, improved, regressed, unchanged
    );

    if !abs_deltas.is_empty() {
        print_range("    change:  ", &abs_deltas, fmt_human, percentile_i64);
    } else {
        println!("    change:   n/a");
    }

    if !percent_diffs.is_empty() {
        print_range("    change %:", &percent_diffs, fmt_percent, percentile_f64);
    } else {
        println!("    change %: n/a");
    }
}

fn print_range<T, F, P>(prefix: &str, values: &[T], format: F, percentile_fn: P)
where
    T: PartialOrd + Copy,
    F: Fn(T) -> String,
    P: Fn(&[T], usize) -> T,
{
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = sorted.first().copied().unwrap();
    let med = percentile_fn(&sorted, 50);
    let max = sorted.last().copied().unwrap();

    println!(
        "{prefix} [min {} | med {} | max {}]",
        format(min),
        format(med),
        format(max),
    );
}

fn percentile_f64(sorted: &[f64], pct: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = pct as f64 / 100.0 * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let weight = rank - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

fn percentile_i64(sorted: &[i64], pct: usize) -> i64 {
    if sorted.is_empty() {
        return 0;
    }
    let rank = pct as f64 / 100.0 * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let weight = rank - lower as f64;
        (sorted[lower] as f64 * (1.0 - weight) + sorted[upper] as f64 * weight).round() as i64
    }
}

fn fmt_human(val: i64) -> String {
    if val == 0 {
        return "0".to_string(); // Don't show sign for zero values.
    }
    let val = val as f64;
    for (divisor, suffix) in &[(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "K")] {
        if val.abs() >= *divisor {
            return format!("{:+.2} {}", val / divisor, suffix);
        }
    }
    format!("{:+.0}", val)
}

fn fmt_percent(value: f64) -> String {
    if value.abs() < 0.01 {
        return format!("{:.2}%", value); // Don't show sign for zero values.
    }
    format!("{:+.2}%", value)
}
