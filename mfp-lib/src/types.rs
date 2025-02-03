#[derive(Debug, Clone)]
pub struct FileProcessingResult {
    /// Number of words in each line
    pub line_counts: Vec<usize>,
    /// Total number of words in the file
    pub total_words: usize,
}
