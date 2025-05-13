fn format_with_unit(val: f64) -> (f64, &'static str) {
    const UNITS: &[(f64, &str)] = &[(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "K")];
    for &(divisor, suffix) in UNITS {
        if val.abs() >= divisor {
            return (val / divisor, suffix);
        }
    }
    (val, "")
}

pub(crate) fn fmt_human_i64(value: i64) -> String {
    if value == 0 {
        return "0".to_string(); // No sign for zero values.
    }
    let (scaled, unit) = format_with_unit(value as f64);
    match unit {
        "" => format!("{scaled:+.0}"),
        _ => format!("{scaled:+.2}{unit}"),
    }
}

pub(crate) fn fmt_human_percent(value: f64) -> String {
    if value.abs() < 0.01 {
        format!("{:.2}%", value)
    } else {
        format!("{:+.2}%", value)
    }
}
