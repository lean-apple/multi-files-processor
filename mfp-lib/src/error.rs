use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during text processing
#[derive(Error, Debug)]
pub enum TextProcessorError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("No files provided to process")]
    EmptyFileList,

    #[error("Failed to process {failed_count} out of {total_count} files")]
    PartialProcessingFailure {
        failed_count: usize,
        total_count: usize,
    },
}
