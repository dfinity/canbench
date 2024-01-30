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
        println!("");
        println!("  {} (profiling):", profile);
        print_measurement(
            &measurement,
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
    let old_value = match old_value {
        Some(old_value) => old_value,
        None => {
            // No old value exists. This is a new metric.
            println!("    {metric}: {value} (new)");
            return;
        }
    };

    match old_value {
        0 => {
            // The old value is zero, so changes cannot be reported as a percentage.
            if value == 0 {
                println!("    {metric}: {value} (no change)",);
            } else {
                println!(
                    "    {}",
                    format!("{metric}: {value} (regressed from 0)").red().bold()
                );
            }
        }
        _ => {
            // The old value is > 0. Report changes as percentages.
            let diff = ((value as f64 - old_value as f64) / old_value as f64) * 100.0;
            if diff == 0.0 {
                println!("    {metric}: {value} (no change)");
            } else if diff.abs() < NOISE_THRESHOLD {
                println!(
                    "    {metric}: {value} ({:.2}%) (change within noise threshold)",
                    diff
                );
            } else if diff > 0.0 {
                println!(
                    "    {}",
                    format!("{}: {value} (regressed by {:.2}%)", metric, diff,)
                        .red()
                        .bold()
                );
            } else {
                println!(
                    "    {}",
                    format!("{}: {value} (improved by {:.2}%)", metric, diff.abs(),)
                        .green()
                        .bold()
                );
            }
        }
    }
}
