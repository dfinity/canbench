use crate::data::{Change, Entry};

pub(crate) fn filter_entries(data: &[Entry], noise_threshold: f64) -> Vec<Entry> {
    let mut filtered: Vec<Entry> = data
        .iter()
        .filter_map(|entry| {
            let metrics = [
                &entry.instructions,
                &entry.heap_increase,
                &entry.stable_memory_increase,
            ];

            let is_significant = metrics.iter().any(|v| {
                matches!(
                    v.status(noise_threshold),
                    Change::New | Change::Improved | Change::Regressed
                )
            });

            if !is_significant {
                return None;
            }

            let mut status = String::new();
            if entry.status.is_empty() {
                if metrics
                    .iter()
                    .any(|v| v.status(noise_threshold) == Change::Regressed)
                {
                    status.push('+');
                }
                if metrics
                    .iter()
                    .any(|v| v.status(noise_threshold) == Change::Improved)
                {
                    if !status.is_empty() {
                        status.push('/');
                    }
                    status.push('-');
                }
            } else {
                status = entry.status.clone();
            }

            let mut updated = entry.clone();
            updated.status = status;
            Some(updated)
        })
        .collect();

    // Sort by name, ascending.
    filtered.sort_by(|a, b| a.benchmark.full_name().cmp(&b.benchmark.full_name()));
    // Sort by status.
    filtered.sort_by(|a, b| a.status.cmp(&b.status));
    // Sort by instructions percent diff, descending.
    const EMPTY: f64 = f64::MIN;
    filtered.sort_by(|a, b| {
        a.instructions
            .percent_diff()
            .unwrap_or(EMPTY)
            .partial_cmp(&b.instructions.percent_diff().unwrap_or(EMPTY))
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });

    filtered
}

pub(crate) fn print_table(data: &[Entry], max_displayed_rows: usize) {
    let columns = [
        "status", "name", "ins", "ins Δ%", "HI", "HI Δ%", "SMI", "SMI Δ%",
    ];

    let mut rows = Vec::new();
    for entry in data {
        let name = entry.benchmark.full_name();
        let row = [
            entry.status.clone(),
            name,
            entry.instructions.fmt_human_current(), // Table report use short numbers
            entry.instructions.fmt_human_percent(),
            entry.heap_increase.fmt_human_current(),
            entry.heap_increase.fmt_human_percent(),
            entry.stable_memory_increase.fmt_human_current(),
            entry.stable_memory_increase.fmt_human_percent(),
        ];
        rows.push(row);
    }

    // Calculate max column widths
    let mut col_widths = columns.iter().map(|h| h.len()).collect::<Vec<_>>();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }

    // Helper to print a row
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

    // Print separator
    print!("|");
    for width in &col_widths {
        print!("{}|", "-".repeat(width + 2));
    }
    println!();

    let total_rows = rows.len();
    if total_rows <= max_displayed_rows {
        for row in &rows {
            print_row(row);
        }
    } else {
        let half_limit = max_displayed_rows / 2;
        for row in &rows[..half_limit] {
            print_row(row);
        }

        // Print omitted rows indicator
        let mut omitted_row = vec!["".to_string(); columns.len()];
        omitted_row[0] = "...".to_string(); // Set in "status" column
        omitted_row[1] = format!("({} omitted)", total_rows - max_displayed_rows); // Set in "name" column
        print_row(&omitted_row);

        for row in &rows[total_rows - half_limit..] {
            print_row(row);
        }
    }
}
