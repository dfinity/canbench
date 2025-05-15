use crate::data::Entry;
use std::{fs::File, io::Write, path::PathBuf};

/// Delimiter used in the CSV file.
/// Use `,` for GitHub/VSCode preview.
/// Use `\t` for better compatibility with Google Sheets.
const DELIMITER: char = ',';

/// Write benchmark results to a CSV file.
pub(crate) fn write(output_file: &PathBuf, data: &[Entry]) {
    let mut file = File::create(output_file)
        .unwrap_or_else(|e| panic!("Failed to create results file {:?}: {}", output_file, e));

    const HEADERS: &[&str] = &[
        "status",
        "name",
        "instructions",
        "instructions Δ",
        "instructions Δ%",
        "heap_increase",
        "heap_increase Δ",
        "heap_increase Δ%",
        "stable_memory_increase",
        "stable_memory_increase Δ",
        "stable_memory_increase Δ%",
    ];

    writeln!(file, "{}", HEADERS.join(&DELIMITER.to_string())).expect("Failed to write CSV header");

    for entry in data {
        let name = entry.benchmark.full_name();
        let row = [
            entry.status.clone(),
            name.clone(),
            // CSV report use full numbers
            entry.instructions.fmt_current(),
            entry.instructions.fmt_abs_delta(),
            entry.instructions.fmt_percent(),
            entry.heap_increase.fmt_current(),
            entry.heap_increase.fmt_abs_delta(),
            entry.heap_increase.fmt_percent(),
            entry.stable_memory_increase.fmt_current(),
            entry.stable_memory_increase.fmt_abs_delta(),
            entry.stable_memory_increase.fmt_percent(),
        ];

        writeln!(file, "{}", row.join(&DELIMITER.to_string()))
            .unwrap_or_else(|e| panic!("Failed to write row for {}: {}", name, e));
    }
}
