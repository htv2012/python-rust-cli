use blkrs::run_lsblk;
use clap::{Arg, ColorChoice, Command};

fn main() {
    let matches = Command::new("lsblk")
        .version("0.0.1")
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
            Some(found) => match serde_json::to_string(&found) {
                Ok(output) => println!("{}", output),
                Err(message) => {
                    println!("{}", found);
                    eprintln!("{}", message);
                }
            },
            None => {
                eprintln!("Device not found: '{}'", device);
            }
        }
    }
}
