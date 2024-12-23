// main.rs
// mod time_tool;
use std::time::{Duration, SystemTime};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// Time stamp (as integer)
    #[arg()]
    delta: Option<u64>,
}

fn main() {
    let parsed = Cli::parse();
    println!("parsed={:?}", parsed);
    let delta = Duration::new(parsed.delta.unwrap_or(0), 0);
    println!("delta = {:?}", delta);

    let now = SystemTime::now();
    println!("now = {:?}", now);
    let then = now.checked_add(delta);
    println!("then = {:?}", then);

    // TODO: Format as string
}
