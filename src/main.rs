// src/main.rs
/*
 * Main executable for NeoOptic
 */

use clap::Parser;
use neooptic::{Result, run};

#[derive(Parser)]
#[command(version, about = "NeoOptic - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
