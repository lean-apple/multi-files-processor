use crate::error::TextProcessorError;
use crate::types::FileProcessingResult;
use crate::utils::{count_words, validate_file_path};
use futures::future;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{error, info};

#[derive(Debug, Default)]
pub struct TextProcessor {
    results: HashMap<PathBuf, FileProcessingResult>,
}

impl TextProcessor {
    /// Creates a new TextProcessor instance
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    /// Processes multiple files concurrently
    pub async fn process_files(
        &mut self,
        file_paths: Vec<PathBuf>,
    ) -> Result<(), TextProcessorError> {
        if file_paths.is_empty() {
            return Err(TextProcessorError::EmptyFileList);
        }

        info!("Starting to process {} files", file_paths.len());

        let tasks: Vec<_> = file_paths
            .into_iter()
            .map(|path| async {
                let result = self.process_single_file(path.clone()).await;
                (path, result)
            })
            .collect();

        let results = future::join_all(tasks).await;
        let total_count = results.len();
        let mut failed_count = 0;

        // Handle results and populate the results
        for (path, result) in results {
            match result {
                Ok(file_result) => {
                    info!("Successfully processed file: {:?}", path);
                    self.results.insert(path, file_result);
                }
                Err(e) => {
                    failed_count += 1;
                    error!("Error processing file: {}", e);
                }
            }
        }

        if failed_count > 0 {
            error!(
                "Failed to process {} out of {} files",
                failed_count, total_count
            );
            return Err(TextProcessorError::PartialProcessingFailure {
                failed_count,
                total_count,
            });
        }
        info!("Successfully processed all {} files", total_count);
        Ok(())
    }

    /// Processes a single file asynchronously
    async fn process_single_file(
        &self,
        file_path: PathBuf,
    ) -> Result<FileProcessingResult, TextProcessorError> {
        if (validate_file_path(&file_path).await).is_err() {
            return Err(TextProcessorError::FileNotFound(file_path));
        }

        let file = File::open(&file_path)
            .await
            .map_err(TextProcessorError::IoError)?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut line_counts = Vec::new();
        let mut total_words = 0;

        while let Some(line) = lines.next_line().await? {
            let word_count = count_words(&line);
            total_words += word_count;
            line_counts.push(word_count);
        }

        Ok(FileProcessingResult {
            line_counts,
            total_words,
        })
    }

    /// Returns a reference to the processing results
    pub fn get_results(&self) -> &HashMap<PathBuf, FileProcessingResult> {
        &self.results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create temporary test files with given content
    async fn create_test_file(dir: &tempfile::TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(filename);
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    // Verify new processor starts with empty results
    #[tokio::test]
    async fn test_new_processor_creates_empty_results() {
        let processor = TextProcessor::new();
        assert!(processor.get_results().is_empty());
    }

    // Test empty file handling
    #[tokio::test]
    async fn test_process_empty_file_returns_zero_counts() {
        let temp = TempDir::new().unwrap();
        let file_path = create_test_file(&temp, "empty.txt", "").await;

        let processor = TextProcessor::new();
        let result = processor
            .process_single_file(file_path.clone())
            .await
            .unwrap();

        assert_eq!(result.line_counts.len(), 0);
        assert_eq!(result.total_words, 0);
    }

    // Test word counting across multiple lines
    #[tokio::test]
    async fn test_process_multiline_file_counts_correctly() {
        let temp = TempDir::new().unwrap();
        let content = "one two\nthree four five\nsix";
        let file_path = create_test_file(&temp, "multi.txt", content).await;

        let processor = TextProcessor::new();
        let result = processor
            .process_single_file(file_path.clone())
            .await
            .unwrap();

        assert_eq!(result.line_counts, vec![2, 3, 1]);
        assert_eq!(result.total_words, 6);
    }

    // Verify error handling for non-existent files
    #[tokio::test]
    async fn test_nonexistent_file_returns_error() {
        let processor = TextProcessor::new();
        let result = processor
            .process_single_file(PathBuf::from("nonexistent.txt"))
            .await;

        assert!(matches!(result, Err(TextProcessorError::FileNotFound(_))));
    }

    // Test handling of empty input file list
    #[tokio::test]
    async fn test_empty_input_returns_error() {
        let mut processor = TextProcessor::new();
        let result = processor.process_files(vec![]).await;
        assert!(matches!(result, Err(TextProcessorError::EmptyFileList)));
    }

    // Test concurrent processing of multiple files
    #[tokio::test]
    async fn test_process_multiple_files_successful() {
        let temp = TempDir::new().unwrap();
        let file1 = create_test_file(&temp, "file1.txt", "one two").await;
        let file2 = create_test_file(&temp, "file2.txt", "three").await;

        let mut processor = TextProcessor::new();
        let result = processor
            .process_files(vec![file1.clone(), file2.clone()])
            .await;

        assert!(result.is_ok());
        let results = processor.get_results();
        assert_eq!(results.len(), 2);
        assert_eq!(results.get(&file1).unwrap().total_words, 2);
        assert_eq!(results.get(&file2).unwrap().total_words, 1);
    }

    // Test partial success when processing mix of valid and invalid files
    #[tokio::test]
    async fn test_partial_processing_failure() {
        let temp = TempDir::new().unwrap();
        let valid_file = create_test_file(&temp, "valid.txt", "content").await;
        let invalid_file = PathBuf::from("nonexistent.txt");

        let mut processor = TextProcessor::new();
        let result = processor
            .process_files(vec![valid_file.clone(), invalid_file])
            .await;

        assert!(matches!(
            result,
            Err(TextProcessorError::PartialProcessingFailure {
                failed_count: 1,
                total_count: 2,
            })
        ));
        assert_eq!(processor.get_results().len(), 1);
    }
}
