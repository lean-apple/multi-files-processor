# Multi files processor

Concurrent word counter for multiple text files, providing both a library and CLI tool.

## Prerequisites

- [Rust](https://www.rust-lang.org/fr/tools/install)

## CLI use

### Installation

- Inside this repository
```bash
cargo install --path mfp-cli
```
- or to use in a dev mode

```bash 
cargo run --bin mfp-cli [OPTIONS] [FILES]
```

### Options

- `--format`, `-f` <FORMAT>: Output format (text/json)
- `--verbose`, `-v` : Show more detailed figures including total word counts

### Examples

```bash 
# Process single file
cargo run --bin  mfp-cli mfp-lib/tests/files/initial1.txt

# Process multiple files with JSON output
cargo run --bin mfp-cli -- --format json mfp-lib/tests/files/longer.txt mfp-lib/tests/files/unicode.txt

# Show detailed figures with --verbose flag - order matters - simple output
cargo run --bin mfp-cli -- --verbose  mfp-lib/tests/files/*.txt 

# Show detailed figures with --verbose flag - order matters - with JSON output
cargo run --bin mfp-cli -- --format json --verbose  mfp-lib/tests/files/*.txt 
```

### Output examples:

- Text format :

```text
Processing Results:
------------------
longer.txt: 180 words total
  Line counts: [11, 14, 12, 14, 12, 12, 11, 14, 15, 16, 12, 12, 14, 11]
initial_1.txt: 5 words total
  Line counts: [2, 3]
unicode.txt: 10 words total
  Line counts: [2, 3, 5]
initial_2.txt: 4 words total
  Line counts: [2, 2]
larger_spaces.txt: 12 words total
  Line counts: [0, 5, 1, 0, 0, 2, 2, 2]
empty.txt: 0 words total
  Line counts: []
```

- JSON format (verbose):
```json
{
  "files": {
    "initial_1.txt": {
      "line_counts": [2, 3],
      "total_words": 5
    }
  }
}
```
## Library use

```rust 

use mfp_lib::TextProcessor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut processor = TextProcessor::new();
    
    // Process files
    processor.process_files(vec!["file1.txt".into()]).await?;
    
    // Get results
    let results = processor.get_results();
    println!("{:?}", results);
    
    Ok(())
}
```
