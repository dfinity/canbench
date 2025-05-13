fn format_with_unit(val: f64) -> (f64, &'static str) {
    const UNITS: &[(f64, &str)] = &[(1e12, "T"), (1e9, "B"), (1e6, "M"), (1e3, "K")];
    for &(divisor, suffix) in UNITS {
        if val.abs() >= divisor {
            return (val / divisor, suffix);
        }
    }
    (val, "")
}

pub(crate) fn fmt_current(value: u64) -> String {
    let (value, unit) = format_with_unit(value as f64);
    if unit.is_empty() {
        return format!("{value}");
    }
    format!("{value:.2}{unit}")
}

pub(crate) fn fmt_change(value: i64) -> String {
    if value == 0 {
        return String::from("0"); // Don't show sign for zero values.
    }
    let (value, unit) = format_with_unit(value as f64);
    if unit.is_empty() {
        return format!("{value:+.0}");
    }
    format!("{value:+.2}{unit}")
}

pub(crate) fn fmt_percent(value: f64) -> String {
    if value.abs() < 0.01 {
        return format!("{:.2}%", value); // Don't show sign for zero values.
    }
    format!("{:+.2}%", value)
}
