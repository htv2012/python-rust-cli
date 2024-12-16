use blkrs::run_lsblk;

fn main() {
    let matches: clap::ArgMatches = clap::Command::new("lsblk")
        .version("0.0.1")
        .author("Hai Vu")
        .about("lsblk in Rust")
        .color(clap::ColorChoice::Always)
        .arg(
            clap::Arg::new("device")
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
