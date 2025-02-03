mod args;
mod error;
mod format;

use args::Cli;
use clap::Parser;
use error::CliError;
use format::format_output;
use mfp_lib::TextProcessor;
use std::process;
use tracing::{error, info};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    fmt::init();

    // Parse and validate command line arguments
    let args = Cli::parse();

    if let Err(e) = args.validate() {
        error!("{}", e);
        process::exit(1);
    }

    info!("Starting to process {} files", args.files.len());

    // Process files
    let mut processor = TextProcessor::new();
    processor
        .process_files(args.files)
        .await
        .map_err(|e| CliError::InputError(format!("Failed to process files: {}", e)))?;

    format_output(processor.get_results(), args.format, args.verbose)
        .map_err(|e| CliError::FormatError(format!("Failed to format output: {}", e)))?;

    Ok(())
}
