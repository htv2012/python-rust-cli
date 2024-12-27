// main.rs
mod arguments;
mod device;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::OpenOptions;

fn main() {
    let parsed = arguments::parse();

    let mut builder = Builder::new();
    builder.filter_level(if parsed.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });

    if parsed.log_to_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("blkrs.log")
            .unwrap();
        builder.target(Target::Pipe(Box::new(file)));
    }

    builder.init();

    log::debug!("debug = {}", parsed.debug);
    log::debug!("verbosity level = {}", parsed.verbosity);

    match &parsed.command {
        arguments::Commands::Info { device } => match device::run_lsblk(device) {
            Ok(json_value) => match serde_json::to_string_pretty(&json_value) {
                Ok(json_string) => {
                    println!("{}", json_string);
                }
                Err(msg) => {
                    println!("{}", json_value);
                    eprintln!("{}", msg);
                }
            },
            Err(msg) => {
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        },
    }
}
