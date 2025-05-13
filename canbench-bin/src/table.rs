use crate::data::Entry;

const DELIMITER: &str = " | ";

pub(crate) fn print_table(data: &[Entry]) {
    let columns = [
        "status", "name", "ins", "ins Δ%", "HI", "HI Δ%", "SMI", "SMI Δ%",
    ];

    let mut rows = Vec::new();
    for entry in data {
        let name = entry.benchmark.full_name();
        let row = [
            entry.status.clone(),
            name,
            entry.instructions.fmt_current(),
            entry.instructions.fmt_percent(),
            entry.heap_increase.fmt_current(),
            entry.heap_increase.fmt_percent(),
            entry.stable_memory_increase.fmt_current(),
            entry.stable_memory_increase.fmt_percent(),
        ];
        rows.push(row);
    }

    // Calculate max column widths
    let mut col_widths = columns
        .iter()
        .map(|header| header.len())
        .collect::<Vec<_>>();

    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }

    // Helper to print a row with correct alignment
    let print_row = |row: &[String]| {
        for (i, cell) in row.iter().enumerate() {
            let width = col_widths[i];
            match i {
                0 => print!(
                    "{:^width$}{}",
                    cell,
                    if i != row.len() - 1 { DELIMITER } else { "" },
                    width = width
                ), // Center status
                1 => print!(
                    "{:<width$}{}",
                    cell,
                    if i != row.len() - 1 { DELIMITER } else { "" },
                    width = width
                ), // Left-align name
                _ => print!(
                    "{:>width$}{}",
                    cell,
                    if i != row.len() - 1 { DELIMITER } else { "" },
                    width = width
                ), // Right-align numbers
            }
        }
        println!();
    };

    // Print header
    print_row(&columns.iter().map(|s| s.to_string()).collect::<Vec<_>>());

    // Print separator line
    let total_width: usize =
        col_widths.iter().sum::<usize>() + DELIMITER.len() * (columns.len() - 1);
    println!("{}", "-".repeat(total_width));

    // Print rows
    for row in &rows {
        print_row(row);
    }
}
