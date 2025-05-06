use canbench_rs::{BenchResult, Measurement};
use std::collections::BTreeMap;

pub(crate) fn print_stats(
    new: &BTreeMap<String, BenchResult>,
    old: &BTreeMap<String, BenchResult>,
    noise_threshold: f64,
) {
    print_percentage_stats("Instructions", new, old, noise_threshold, |m| {
        m.instructions
    });
    print_percentage_stats("Heap Increase (pages)", new, old, noise_threshold, |m| {
        m.heap_increase
    });
    print_percentage_stats(
        "Stable Memory Increase (pages)",
        new,
        old,
        noise_threshold,
        |m| m.stable_memory_increase,
    );
}

fn print_percentage_stats<F>(
    label: &str,
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
    noise_threshold: f64,
    extractor: F,
) where
    F: Fn(&Measurement) -> u64,
{
    let mut changed = 0;
    let mut unchanged = 0;
    let mut new_only = 0;

    let mut abs_deltas = Vec::new();
    let mut percent_diffs = Vec::new();

    for (name, new) in new_results {
        let new_val = extractor(&new.total);
        match old_results.get(name) {
            Some(old) => {
                let old_val = extractor(&old.total);
                let abs_delta = new_val as i64 - old_val as i64;
                abs_deltas.push(abs_delta);

                match old_val {
                    0 if abs_delta != 0 => changed += 1,
                    0 => {
                        unchanged += 1;
                        percent_diffs.push(0.0);
                    }
                    _ => {
                        let delta = abs_delta as f64 / old_val as f64 * 100.0;
                        if delta.abs() >= noise_threshold {
                            changed += 1;
                        } else {
                            unchanged += 1;
                        }
                        percent_diffs.push(delta);
                    }
                }
            }
            None => {
                new_only += 1;
                abs_deltas.push(new_val as i64);
            }
        }
    }

    let total = changed + unchanged + new_only;

    println!("=== {label} ===");
    println!("Changed: {changed}, Unchanged: {unchanged}, New: {new_only}, Total: {total}");
    println!("Noise threshold: {:.1}%", noise_threshold);

    if !abs_deltas.is_empty() {
        print_range("Change   ", &abs_deltas, fmt_human, percentile_i64);
    } else {
        println!("Change   : n/a");
    }

    if !percent_diffs.is_empty() {
        print_range(
            "Change % ",
            &percent_diffs,
            |v| format!("{:+.1}%", v),
            percentile_f64,
        );
    } else {
        println!("Change % : n/a");
    }

    println!();
}

fn print_range<T, F, P>(label: &str, values: &[T], format: F, percentile_fn: P)
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
        "{label}: [min {} | med {} | max {}]",
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
    let val = val as f64;
    for (divisor, suffix) in &[(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "K")] {
        if val.abs() >= *divisor {
            return format!("{:.2} {}", val / divisor, suffix);
        }
    }
    format!("{}", val)
}
