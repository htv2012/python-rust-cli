// main.rs
use clap::Parser;
use std::time::Duration;
use std::time::SystemTime;
mod cli;
mod time_tool;

const DEFAULT_FORMAT: &str = "rfc3339";
const RFC_3339: &str = "%Y-%m-%d %H:%M:%S";

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
