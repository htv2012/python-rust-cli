// main.rs
mod arguments;
mod device;
use env_logger::Builder;
use log::LevelFilter;

fn main() {
    let parsed = arguments::parse();
    Builder::new()
        .filter_level(if parsed.debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();

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
            }
        },
    }
}
