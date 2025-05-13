use crate::data::Entry;

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

    // Helper to print a row with correct alignment and separators
    let print_row = |row: &[String]| {
        print!("|");
        for (i, cell) in row.iter().enumerate() {
            let width = col_widths[i];
            match i {
                0 => print!(" {:^width$} ", cell, width = width), // Center status
                1 => print!(" {:<width$} ", cell, width = width), // Left-align name
                _ => print!(" {:>width$} ", cell, width = width), // Right-align numbers
            }
            print!("|");
        }
        println!();
    };

    // Print header
    print_row(&columns.iter().map(|s| s.to_string()).collect::<Vec<_>>());

    // Print separator line
    print!("|");
    for (i, width) in col_widths.iter().enumerate() {
        print!("{}|", "-".repeat(width + 2));
    }
    println!();

    // Print data rows
    for row in &rows {
        print_row(row);
    }
}
