use std::io::Error;
use std::path::Path;
use tokio::fs;

/// Counts the number of words in a line by splitting on whitespace
/// TODO: check to replace with `unicode_segmentation crate`
/// to have a better word counting
pub fn count_words(line: &str) -> usize {
    line.split_whitespace().count()
}

/// Validates that a file exists and is readable
pub async fn validate_file_path(path: &Path) -> Result<(), Error> {
    fs::metadata(path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_counting() {
        assert_eq!(count_words("Hello world!"), 2);
        assert_eq!(count_words("emoji test: 🌟 💻 🚀"), 5);
        assert_eq!(count_words("こんにちは world !"), 3);
        assert_eq!(count_words("Café and résumé"), 3);
        assert_eq!(count_words("  multiple   spaces  "), 2);
        assert_eq!(count_words("hyphenated-word"), 1);
        assert_eq!(count_words("!@#$ symbols"), 2);
    }
}
