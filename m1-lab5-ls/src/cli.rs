use clap::{Arg, ArgMatches, ColorChoice, Command};

pub fn get_target() -> String {
    let matches: ArgMatches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hai Vu")
        .about("List files in a directory")
        .color(ColorChoice::Auto)
        .arg(
            Arg::new("dir")
                .help("Path to dir")
                .required(false)
                .index(1)
                .default_value("."),
        )
        .get_matches();

    matches.get_one::<String>("dir").unwrap().clone()
}
