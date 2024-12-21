// cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
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
