use crate::data::Entry;
use std::io::Write;

/// Delimiter used in the CSV file.
/// Use `,` for GitHub/VSCode preview.
/// Use `\t` for better compatibility with Google Sheets.
const DELIMITER: char = ',';

/// Write benchmark results to a CSV file.
pub(crate) fn write<W: Write>(writer: &mut W, data: &[Entry]) -> std::io::Result<()> {
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

    writeln!(writer, "{}", HEADERS.join(&DELIMITER.to_string()))?;

    for entry in data {
        let name = entry.benchmark.full_name();
        let row = [
            entry.status.clone(),
            name.clone(),
            // CSV report uses full numbers
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

        writeln!(writer, "{}", row.join(&DELIMITER.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Benchmark, Values};

    fn run_write_csv_case(entries: &[Entry], expected_output: &str) {
        let mut output = Vec::new();
        let _ = write(&mut output, entries);

        let output_str = String::from_utf8_lossy(&output);
        assert_eq!(
            output_str, expected_output,
            "Unexpected output:\n{}",
            output_str
        );
    }

    #[test]
    fn test_write_csv() {
        run_write_csv_case(
&[
            Entry {
                status: "".to_string(),
                benchmark: Benchmark::new("bench_1", None),
                instructions: Values::new(Some(11_000_000), Some(10_000_000)),
                heap_increase: Values::new(Some(0), None),
                stable_memory_increase: Values::new(Some(0), None),
            },
            Entry {
                status: "".to_string(),
                benchmark: Benchmark::new("bench_2", None),
                instructions: Values::new(Some(9_000_000), Some(10_000_000)),
                heap_increase: Values::new(Some(0), None),
                stable_memory_increase: Values::new(Some(0), None),
            },
        ],
            "\
status,name,instructions,instructions Δ,instructions Δ%,heap_increase,heap_increase Δ,heap_increase Δ%,stable_memory_increase,stable_memory_increase Δ,stable_memory_increase Δ%
,bench_1,11000000,1000000,10.00%,0,,,0,,
,bench_2,9000000,-1000000,-10.00%,0,,,0,,
"
        );
    }
}
