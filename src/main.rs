// src/main.rs
/*
 * Main executable for BlockchainledgerSynchronizer
 */

use clap::Parser;
use blockchainledgersynchronizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainledgerSynchronizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
