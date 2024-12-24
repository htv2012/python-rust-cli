// time_toool.rs
use chrono::{DateTime, Local};
use std::time::SystemTime;

/// Format a `stamp` based on the `format_spec`
pub fn format_time(stamp: &SystemTime, format_spec: &str) -> String {
    let date_time: DateTime<Local> = (*stamp).into();
    let formatted = date_time.format(format_spec).to_string();
    formatted
}
