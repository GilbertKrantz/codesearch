//! Minimal parser utilities: file I/O and path helpers.
//!
//! For regex-based code extraction (functions, classes, references), use the `extract` module.

use std::fs;
use std::path::Path;

/// Read file content, returning empty string on error
pub fn read_file_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_default()
}

/// Extract file extension from path
pub fn get_file_extension(file_path: &str) -> &str {
    Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_content() {
        let content = read_file_content("nonexistent.rs");
        assert_eq!(content, "");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("test.rs"), "rs");
        assert_eq!(get_file_extension("test.py"), "py");
        assert_eq!(get_file_extension("test"), "");
    }
}
