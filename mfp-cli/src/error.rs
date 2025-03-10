use std::error::Error;
use std::fmt;

/// Errors occuring during CLI operations
#[derive(Debug)]
pub enum CliError {
    /// Input files or processing errors
    InputError(String),
    /// Output formatting errors
    FormatError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InputError(msg) => write!(f, "Input error: {}", msg),
            CliError::FormatError(msg) => write!(f, "Format error: {}", msg),
        }
    }
}

impl Error for CliError {}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::FormatError(err.to_string())
    }
}
