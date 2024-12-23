// main.rs
// mod time_tool;
use chrono::{DateTime, Local};
use std::time::{Duration, SystemTime};

use clap::Parser;

const DEFAULT_FORMAT: &str = "rfc3339";
const RFC_3339: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Parser, Debug)]
struct Cli {
    /// Format string, e.g. 'rfc3339' (default) or '%Y-%m%d %H:%M:%S'
    #[clap(long, short)]
    format: Option<String>,

    /// Number of seconds into the future (as integer)
    #[clap()]
    delta: Option<u64>,
}

/// Format a `stamp` based on the `format_spec`
fn format_time(stamp: &SystemTime, format_spec: &str) -> String {
    let date_time: DateTime<Local> = (*stamp).into();
    let formatted = date_time.format(format_spec).to_string();
    formatted
}

fn main() {
    let parsed = Cli::parse();
    let delta = Duration::new(parsed.delta.unwrap_or(0), 0);
    let mut  format = parsed.format.unwrap_or(RFC_3339.to_string());

    if format == DEFAULT_FORMAT {
        format = RFC_3339.to_string();
    }

    let now = SystemTime::now();
    let then = now.checked_add(delta).unwrap_or(now);
    println!("{}", format_time(&then, &format));
}
