// src/main.rs
/*
 * Main executable for CipherGuard2
 */

use clap::Parser;
use cipherguard2::{Result, run};

#[derive(Parser)]
#[command(version, about = "CipherGuard2 - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
