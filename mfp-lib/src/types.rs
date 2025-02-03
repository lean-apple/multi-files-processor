use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileProcessingResult {
    /// Path to the processed file
    pub file_path: PathBuf,
    /// Number of words in each line
    pub line_counts: Vec<usize>,
    /// Total number of words in the file
    pub total_words: usize,
}
