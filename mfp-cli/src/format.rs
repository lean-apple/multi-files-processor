use crate::error::CliError;
use clap::ValueEnum;
use mfp_lib::FileProcessingResult;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::debug;

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Simple text output with line-by-line counts
    Text,
    /// Structured JSON format
    Json,
}

/// Outputs the processing results in the specified format
pub fn format_output(
    results: &HashMap<PathBuf, FileProcessingResult>,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), CliError> {
    match format {
        OutputFormat::Json => format_json(results, verbose),
        OutputFormat::Text => format_text(results, verbose),
    }
}

#[derive(serde::Serialize)]
struct FileResult {
    line_counts: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_words: Option<usize>,
}

#[derive(serde::Serialize)]
struct OutputResult {
    files: HashMap<String, FileResult>,
}

// Helper to format results as text,
// optionally including total word counts
fn format_text(
    results: &HashMap<PathBuf, FileProcessingResult>,
    verbose: bool,
) -> Result<(), CliError> {
    debug!("Formatting as text");
    println!("\nProcessing Results:");
    println!("------------------");

    for (path, result) in results {
        let filename = path.file_name().unwrap_or_default().to_string_lossy();

        if verbose {
            println!(
                "{}: {} words in total\n  Line counts: {:?}",
                filename, result.total_words, result.line_counts
            );
        } else {
            println!("{}: {:?}", filename, result.line_counts);
        }
    }

    Ok(())
}

// Helper to formats results as JSON,
// optionally including total word counts
fn format_json(
    results: &HashMap<PathBuf, FileProcessingResult>,
    verbose: bool,
) -> Result<(), CliError> {
    debug!("Formatting as JSON");
    let files = results
        .iter()
        .map(|(path, result)| {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            debug!(
                "Processing file {} with total_words={}",
                name, result.total_words
            );

            let file_result = if verbose {
                FileResult {
                    line_counts: result.line_counts.clone(),
                    total_words: Some(result.total_words),
                }
            } else {
                FileResult {
                    line_counts: result.line_counts.clone(),
                    total_words: None,
                }
            };

            (name, file_result)
        })
        .collect();

    let output = OutputResult { files };
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
