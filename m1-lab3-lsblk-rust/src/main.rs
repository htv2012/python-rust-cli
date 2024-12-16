use blkrs::run_lsblk;
use clap::{Arg, ArgMatches, ColorChoice, Command};

fn main() {
    let matches: ArgMatches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hai Vu")
        .about("lsblk in Rust")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(device) = matches.get_one::<String>("device") {
        match run_lsblk(device) {
            Ok(found) => match serde_json::to_string_pretty(&found) {
                Ok(output) => {
                    println!("{}", output);
                }
                Err(message) => {
                    println!("{}", found);
                    eprintln!("{}", message);
                }
            },
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
}
