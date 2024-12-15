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
        let output = serde_json::to_string(&run_lsblk(device)).unwrap();
        println!("{}", output);
    }
}
