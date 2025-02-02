use std::io::Error;
use std::path::Path;
use tokio::fs;

/// Counts the number of words in a line by splitting on whitespace
pub fn count_words(line: &str) -> usize {
    line.split_whitespace().count()
}

/// Validates that a file exists and is readable
pub async fn validate_file_path(path: &Path) -> Result<(), Error> {
    fs::metadata(path).await?;
    Ok(())
}
