//! Handles Command Line Interface (CLI) arguments
use clap::Parser;

/// Command line parameters which this crate can handle
#[derive(Parser, Debug)]
pub struct Cli {
    /// Format string, e.g. 'rfc3339' (default) or '%Y-%m%d %H:%M:%S'
    #[clap(long, short)]
    pub format: Option<String>,

    /// Number of seconds into the future (as integer)
    #[clap()]
    pub delta: Option<u64>,
}
