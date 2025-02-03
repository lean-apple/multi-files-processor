use crate::format::OutputFormat;
use clap::Parser;
use std::path::PathBuf;
use tracing::error;

#[derive(Parser, Debug)]
#[command(name = "mfp", about = "Multi-files text processor", version)]
pub struct Cli {
    /// Files to process - e.g., 'file1.txt file2.txt'
    #[arg(required = true)]
    pub files: Vec<PathBuf>,

    /// Output format: 'text' by default - shows simple format
    /// 'json' provides structured output
    #[arg(long, short, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Display detailed formatted figures as per-line word counts
    #[arg(long, short)]
    pub verbose: bool,
}

impl Cli {
    /// Validates all input files exist and are readable
    pub fn validate(&self) -> Result<(), String> {
        let invalid_files: Vec<_> = self.files.iter().filter(|path| !path.is_file()).collect();

        if !invalid_files.is_empty() {
            let error_msg = format!(
                "Invalid or non-existent files: {}",
                invalid_files
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            error!(error_msg);
            return Err(error_msg);
        }

        Ok(())
    }
}
