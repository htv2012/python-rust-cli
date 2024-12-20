// cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add files
    Add {
        name: Option<String>,
        uid: Option<u32>,
    },

    /// List device information
    Info { device: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, uid } => {
            println!("Add name='{:?}', uid={:?}", name, uid);
        }
        Commands::Info { device } => match device {
            Some(device_name) => {
                println!("Get info for device '{}'", device_name);
            }
            None => {
                println!("Get info for all devices");
            }
        },
    }
}
