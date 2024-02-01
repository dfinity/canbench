use canbench::{BenchResult, Measurement};
use colored::Colorize;

// The threshold that determines whether or not a change is significant.
const NOISE_THRESHOLD: f64 = 2.0;

/// Prints a benchmark to stdout, comparing it to the previous result if available.
pub(crate) fn print_benchmark(name: &str, new: &BenchResult, old: Option<&BenchResult>) {
    // Print benchmark name.
    if old.is_some() {
        println!("Benchmark: {}", name.bold());
    } else {
        println!("Benchmark: {} {}", name.bold(), "(new)".blue().bold());
    }

    // Print totals.
    println!("  total:");
    print_measurement(&new.total, old.map(|m| &m.total));

    // Print custom profiles.
    for (profile, measurement) in &new.profiling {
        println!();
        println!("  {} (profiling):", profile);
        print_measurement(
            measurement,
            old.map(|m| &m.profiling).and_then(|m| m.get(profile)),
        );
    }
}

// Prints a measurement along with a comparison with the old value if available.
fn print_measurement(new: &Measurement, old: Option<&Measurement>) {
    print_metric(
        "instructions",
        new.instructions,
        old.map(|m| m.instructions),
    );
    print_metric("heap_delta", new.heap_delta, old.map(|m| m.heap_delta));
    print_metric(
        "stable_memory_delta",
        new.stable_memory_delta,
        old.map(|m| m.stable_memory_delta),
    );
}

// Prints a metric along with its percentage change relative to the old value.
fn print_metric(metric: &str, value: u64, old_value: Option<u64>) {
    // Convert value to a more readable representation.
    let value_str = if value < 10_000 {
        format!("{}", value)
    } else if value < 1_000_000 {
        format!("{:.2} K", value as f64 / 1_000.0)
    } else if value < 1_000_000_000 {
        format!("{:.2} M", value as f64 / 1_000_000.0)
    } else if value < 1_000_000_000_000 {
        format!("{:.2} B", value as f64 / 1_000_000_000.0)
    } else {
        format!("{:.2} T", value as f64 / 1_000_000_000_000.0)
    };

    // Add unit to value depending on the metric.
    let value_str = match metric {
        "instructions" => {
            // Don't include a unit with instructions since it's clear from the metric name.
            value_str
        }
        "heap_delta" => format!("{value_str} pages"),
        "stable_memory_delta" => format!("{value_str} pages"),
        other => panic!("unknown metric {}", other),
    };

    let old_value = match old_value {
        Some(old_value) => old_value,
        None => {
            // No old value exists. This is a new metric.
            println!("    {metric}: {value_str} (new)");
            return;
        }
    };

    match old_value {
        0 => {
            // The old value is zero, so changes cannot be reported as a percentage.
            if value == 0 {
                println!("    {metric}: {value_str} (no change)",);
            } else {
                println!(
                    "    {}",
                    format!("{metric}: {value_str} (regressed from 0)")
                        .red()
                        .bold()
                );
            }
        }
        _ => {
            // The old value is > 0. Report changes as percentages.
            let diff = ((value as f64 - old_value as f64) / old_value as f64) * 100.0;
            if diff == 0.0 {
                println!("    {metric}: {value_str} (no change)");
            } else if diff.abs() < NOISE_THRESHOLD {
                println!(
                    "    {metric}: {value_str} ({:.2}%) (change within noise threshold)",
                    diff
                );
            } else if diff > 0.0 {
                println!(
                    "    {}",
                    format!("{}: {value_str} (regressed by {:.2}%)", metric, diff,)
                        .red()
                        .bold()
                );
            } else {
                println!(
                    "    {}",
                    format!("{}: {value_str} (improved by {:.2}%)", metric, diff.abs(),)
                        .green()
                        .bold()
                );
            }
        }
    }
}
