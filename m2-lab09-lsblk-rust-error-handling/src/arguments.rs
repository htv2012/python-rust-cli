// cli.rs
use clap::ArgAction;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[clap(long, short, env = "BLKRS_DEBUG")]
    pub debug: bool,

    #[clap(long, help = "Log to a file")]
    pub log_to_file: bool,

    #[clap(long, short, help = "Verbosity level, e.g -v, -vv, -vvv", action = ArgAction::Count)]
    pub verbosity: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List device information
    Info { device: Option<String> },
}

pub fn parse() -> Arguments {
    Arguments::parse()
}
