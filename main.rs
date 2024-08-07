use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Function to scan for SAFETY comments in Rust files
fn scan_safety_comments(source_folder: &str) {
    // Define regular expressions for SAFETY comments
    let re_single_line = Regex::new(r"//\s*SAFETY:").unwrap();
    let re_multi_line_start = Regex::new(r"///\s*#\s*Safety").unwrap();

    // Traverse each file in the source folder
    for entry in WalkDir::new(source_folder).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs") {
            let file = File::open(entry.path()).expect("Failed to open file");
            let reader = BufReader::new(file);

            // To store the current block of multi-line comments
            let mut in_multi_line_comment = false;
            let mut multi_line_comment_block = String::new();

            // Process each line in the file
            for (line_number, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    // Check for single-line SAFETY comments
                    if re_single_line.is_match(&line) {
                        println!(
                            "Single-line SAFETY comment found in file {} at line {}: {}",
                            entry.path().display(),
                            line_number + 1,
                            line.trim()
                        );
                    }

                    // Check for the start of multi-line SAFETY comment block
                    if re_multi_line_start.is_match(&line) {
                        in_multi_line_comment = true;
                        multi_line_comment_block.clear(); // Clear the previous block content
                        multi_line_comment_block.push_str(line.trim());
                        multi_line_comment_block.push('\n');
                    } else if in_multi_line_comment {
                        // Append lines to the multi-line comment block
                        if line.trim().starts_with("///") || line.trim().is_empty() {
                            multi_line_comment_block.push_str(line.trim());
                            multi_line_comment_block.push('\n');
                        } else {
                            // End of multi-line comment block
                            in_multi_line_comment = false;
                            println!(
                                "Multi-line SAFETY comment block in file {} at line {}:\n{}",
                                entry.path().display(),
                                line_number + 1,
                                multi_line_comment_block
                            );
                        }
                    }
                }
            }

            // If the file ends while still in a multi-line comment block, print it
            if in_multi_line_comment {
                println!(
                    "Multi-line SAFETY comment block in file {} at end of file:\n{}",
                    entry.path().display(),
                    multi_line_comment_block
                );
            }
        }
    }
}

fn main() {
    let source_folder = r"rust-smallvec-19de50108d403efaa7cd979eac3bb97a4432fd4b"; // Update this to the directory containing test.rs
    scan_safety_comments(source_folder);
}
