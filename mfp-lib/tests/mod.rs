use mfp_lib::{TextProcessor, TextProcessorError};
use std::path::PathBuf;

// Test file definitions with their expected results
const TEST_FILES: &[(&str, &[usize], usize)] = &[
    // (filename, line_word_counts, total_words)
    ("initial_1.txt", &[2, 3], 5),
    ("initial_2.txt", &[2, 2], 4),
    (
        "longer.txt",
        &[11, 14, 12, 14, 12, 12, 11, 14, 15, 16, 12, 12, 14, 11],
        180,
    ),
    ("empty.txt", &[], 0),
    ("unicode.txt", &[2, 3, 5], 10),
    ("larger_spaces.txt", &[0, 5, 1, 0, 0, 2, 2, 2], 12),
];

/// Helper function to construct the path to a test asset.
fn asset_path(filename: &str) -> PathBuf {
    PathBuf::from("tests").join("files").join(filename)
}

/// Helper function to verify file processing results
fn verify_file_result(
    results: &std::collections::HashMap<PathBuf, mfp_lib::FileProcessingResult>,
    filename: &str,
    expected_line_counts: &[usize],
    expected_total: usize,
) {
    let file_path = asset_path(filename);
    let result = results
        .get(&file_path)
        .unwrap_or_else(|| panic!("No results found for test file: {filename}"));

    assert_eq!(
        result.line_counts, expected_line_counts,
        "Incorrect line word counts for {filename}"
    );
    assert_eq!(
        result.total_words, expected_total,
        "Incorrect total word count for {filename}"
    );
}

#[tokio::test]
async fn test_concurrent_processing_of_all_files() {
    // Create processor and process all test files
    let file_paths: Vec<PathBuf> = TEST_FILES
        .iter()
        .map(|(name, ..)| asset_path(name))
        .collect();
    let mut processor = TextProcessor::new();

    let result = processor.process_files(file_paths).await;
    assert!(
        result.is_ok(),
        "Failed to process files concurrently: {:?}",
        result
    );

    // Verify results for each test file
    let results = processor.get_results();
    assert_eq!(
        results.len(),
        TEST_FILES.len(),
        "Unexpected number of processed files"
    );

    for &(filename, expected_counts, expected_total) in TEST_FILES {
        verify_file_result(results, filename, expected_counts, expected_total);
    }
}

#[tokio::test]
async fn test_partial_failure_with_nonexistent_file() {
    // Setup test files including a nonexistent one
    let mut file_paths: Vec<PathBuf> = TEST_FILES
        .iter()
        .map(|(name, ..)| asset_path(name))
        .collect();
    let nonexistent_file = "nonexistent.txt";
    file_paths.push(asset_path(nonexistent_file));

    let mut processor = TextProcessor::new();

    // Process files and verify error handling
    let result = processor.process_files(file_paths).await;
    match result {
        Err(TextProcessorError::PartialProcessingFailure {
            failed_count,
            total_count,
        }) => {
            assert_eq!(
                failed_count, 1,
                "Expected exactly one failed file (nonexistent.txt)"
            );
            assert_eq!(
                total_count,
                TEST_FILES.len() + 1,
                "Total count should include all attempted files"
            );
        }
        other => panic!("Expected PartialProcessingFailure, got: {:?}", other),
    }

    // Verify successful results
    let results = processor.get_results();
    assert_eq!(
        results.len(),
        TEST_FILES.len(),
        "Should have results for all valid files"
    );

    // Verify all valid files were processed correctly
    for &(filename, expected_counts, expected_total) in TEST_FILES {
        verify_file_result(results, filename, expected_counts, expected_total);
    }
}
