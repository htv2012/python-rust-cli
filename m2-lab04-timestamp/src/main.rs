//! Main module
//!
//! This crate takes in an integer which represents the number of seconds into the future.
//! It then format the resulting time. If the user specifies a format spec, it will be used,
//! Otherwise, the crate will format the time as 'YYYY-MM-YY HH:MM:SS'.
//!
//! Examples
//!
//!     time-stamp
//!
//!     time-stamp --format 'You need to leave by %H:%M' 600

use clap::Parser;
use std::time::Duration;
use std::time::SystemTime;
mod cli;
mod time_tool;

/// The default time format
const DEFAULT_FORMAT: &str = "rfc3339";

/// The RFC-3339 time format
const RFC_3339: &str = "%Y-%m-%d %H:%M:%S";

/// Take in an optional format spec and the number of seconds into the future and format it
fn main() {
    let args = cli::Cli::parse();
    let delta = Duration::new(args.delta.unwrap_or(0), 0);
    let mut format_spec = args.format.unwrap_or(RFC_3339.to_string());

    if format_spec == DEFAULT_FORMAT {
        format_spec = RFC_3339.to_string();
    }

    let now = SystemTime::now();
    let then = now.checked_add(delta).unwrap_or(now);
    println!("{}", time_tool::format_time(&then, &format_spec));
}
