use canbench_rs::{BenchResult, Measurement};
use std::{collections::BTreeMap, fs::File, io::Write, path::PathBuf};

/// Delimiter used in the CSV file.
/// Use `,` for GitHub/VSCode preview.
/// Use `\t` for better compatibility with Google Sheets.
const DELIMITER: char = ',';

/// Write benchmark results to a CSV file.
pub(crate) fn write(
    output_file: &PathBuf,
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
) {
    let mut file = File::create(output_file)
        .unwrap_or_else(|e| panic!("Failed to create results file {:?}: {}", output_file, e));

    let headers = [
        "status",
        "name",
        "instructions",
        "instructions change %",
        "heap_increase",
        "heap_increase change %",
        "stable_memory_increase",
        "stable_memory_increase change %",
    ];

    writeln!(file, "{}", headers.join(&DELIMITER.to_string())).expect("Failed to write CSV header");

    for (name, new_bench) in new_results {
        let old_bench = old_results.get(name);
        let status = if old_bench.is_some() { "" } else { "new" };
        let old = old_bench.map(|b| &b.total);
        write_measurement_diff(&mut file, status, name, &new_bench.total, old);
    }
}

fn write_measurement_diff(
    file: &mut File,
    status: &str,
    name: &str,
    new: &Measurement,
    old: Option<&Measurement>,
) {
    let format_number = |n: u64| n.to_string();
    let format_percent = |new, old| {
        let abs_delta = new as f64 - old as f64;
        if old == 0 {
            match abs_delta {
                d if d < 0.0 => String::from("-inf%"),
                d if d > 0.0 => String::from("+inf%"),
                _ => String::from("0.00%"),
            }
        } else {
            format!("{:.2}%", abs_delta / old as f64 * 100.0)
        }
    };

    let (instructions_p, heap_increase_p, stable_memory_increase_p) = match old {
        Some(old) => (
            format_percent(new.instructions, old.instructions),
            format_percent(new.heap_increase, old.heap_increase),
            format_percent(new.stable_memory_increase, old.stable_memory_increase),
        ),
        None => (String::new(), String::new(), String::new()),
    };

    let row = [
        status,
        name,
        &format_number(new.instructions),
        &instructions_p,
        &format_number(new.heap_increase),
        &heap_increase_p,
        &format_number(new.stable_memory_increase),
        &stable_memory_increase_p,
    ];

    writeln!(file, "{}", row.join(&DELIMITER.to_string()))
        .unwrap_or_else(|e| panic!("Failed to write row for {}: {}", name, e));
}
