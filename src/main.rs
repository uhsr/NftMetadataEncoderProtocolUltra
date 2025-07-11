// src/main.rs
/*
 * Main executable for NftMetadataEncoderProtocolUltra
 */

use clap::Parser;
use nftmetadataencoderprotocolultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMetadataEncoderProtocolUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
