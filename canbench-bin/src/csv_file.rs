use canbench_rs::{BenchResult, Measurement};
use std::{collections::BTreeMap, fs::File, io::Write, path::PathBuf};

/// Write benchmark results to CSV file.
pub(crate) fn write(
    results_file: &PathBuf,
    new_results: &BTreeMap<String, BenchResult>,
    old_results: &BTreeMap<String, BenchResult>,
    delimiter: char,
) {
    let mut file = File::create(results_file)
        .unwrap_or_else(|e| panic!("Failed to create results file {:?}: {}", results_file, e));

    writeln!(
        file,
        "status{dp}name{dp}instructions{dp}instructions %{dp}heap_increase{dp}heap_increase %{dp}stable_memory_increase{dp}stable_memory_increase %",
        dp=delimiter
    ).expect("Failed to write CSV header");

    for (name, new_bench) in new_results {
        let old_bench = old_results.get(name);

        write_measurement_diff(
            &mut file,
            if old_bench.is_some() { "" } else { "new" },
            name,
            &new_bench.total,
            old_bench.map(|b| &b.total),
            delimiter,
        );
    }
}

fn write_measurement_diff(
    file: &mut File,
    new: &str,
    name: &str,
    new_m: &Measurement,
    old_m: Option<&Measurement>,
    delimiter: char,
) {
    let (instr_pct, heap_pct, stable_pct) = match old_m {
        Some(old) => (
            percent_diff(new_m.instructions, old.instructions),
            percent_diff(new_m.heap_increase, old.heap_increase),
            percent_diff(new_m.stable_memory_increase, old.stable_memory_increase),
        ),
        None => (String::new(), String::new(), String::new()),
    };

    writeln!(
        file,
        "{new}{d}{name}{d}{ins}{d}{ins_p}{d}{heap}{d}{heap_p}{d}{smi}{d}{smi_p}",
        new = new,
        name = name,
        ins = new_m.instructions,
        ins_p = instr_pct,
        heap = new_m.heap_increase,
        heap_p = heap_pct,
        smi = new_m.stable_memory_increase,
        smi_p = stable_pct,
        d = delimiter
    )
    .unwrap_or_else(|e| panic!("Failed to write row for {}: {}", name, e));
}

fn percent_diff(new: u64, old: u64) -> String {
    if old == 0 {
        return String::new();
    }
    let diff = (new as f64 - old as f64) / old as f64 * 100.0;
    format!("{:.2}%", diff)
}
