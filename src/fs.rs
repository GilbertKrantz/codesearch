//! File System Abstraction Module
//!
//! Provides trait-based file system operations for dependency injection and testing.
//! Also includes utilities for directory walking with filtering.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Trait for file system operations
///
/// This trait abstracts file system operations to enable dependency injection
/// and make code more testable by allowing mock implementations.
///
/// # Examples
///
/// ```
/// use codesearch::fs::{FileSystem, RealFileSystem};
/// use std::path::Path;
///
/// let fs = RealFileSystem;
/// let content = fs.read_to_string(Path::new("Cargo.toml"));
/// assert!(content.is_ok());
/// ```
pub trait FileSystem: Send + Sync {
    /// Read the entire contents of a file into a string
    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Read the entire contents of a file into a byte vector
    fn read(&self, path: &Path) -> io::Result<Vec<u8>>;

    /// Write a string to a file, creating it if it doesn't exist
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;

    /// Check if a path exists
    fn exists(&self, path: &Path) -> bool;

    /// Check if a path is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Check if a path is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Read directory entries
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;

    /// Get file metadata
    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata>;

    /// Create a directory and all parent directories
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Remove a file
    fn remove_file(&self, path: &Path) -> io::Result<()>;
}

/// Real file system implementation
///
/// This implementation uses the standard library's file system operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        fs::read_dir(path)?
            .map(|entry| entry.map(|e| e.path()))
            .collect()
    }

    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

/// Mock file system for testing
///
/// This implementation stores files in memory and allows testing without
/// touching the real file system.
///
/// # Examples
///
/// ```
/// use codesearch::fs::{FileSystem, MockFileSystem};
/// use std::path::Path;
///
/// let mut fs = MockFileSystem::new();
/// fs.add_file("test.txt", "Hello, World!");
///
/// let content = fs.read_to_string(Path::new("test.txt")).unwrap();
/// assert_eq!(content, "Hello, World!");
/// ```
#[derive(Debug, Clone, Default)]
pub struct MockFileSystem {
    files: std::collections::HashMap<PathBuf, Vec<u8>>,
}

impl MockFileSystem {
    /// Create a new empty mock file system
    pub fn new() -> Self {
        Self {
            files: std::collections::HashMap::new(),
        }
    }

    /// Add a file to the mock file system
    pub fn add_file(&mut self, path: impl Into<PathBuf>, contents: impl Into<Vec<u8>>) {
        self.files.insert(path.into(), contents.into());
    }

    /// Add a text file to the mock file system
    pub fn add_text_file(&mut self, path: impl Into<PathBuf>, contents: &str) {
        self.add_file(path, contents.as_bytes().to_vec());
    }

    /// Check if a file exists in the mock file system
    pub fn has_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// Get the number of files in the mock file system
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

impl FileSystem for MockFileSystem {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
            .and_then(|bytes| {
                String::from_utf8(bytes.clone())
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn write(&self, _path: &Path, _contents: &str) -> io::Result<()> {
        // Mock implementation - would need interior mutability for real use
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn is_dir(&self, _path: &Path) -> bool {
        false // Simplified for mock
    }

    fn read_dir(&self, _path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(self.files.keys().cloned().collect())
    }

    fn metadata(&self, path: &Path) -> io::Result<fs::Metadata> {
        if self.files.contains_key(path) {
            // Return real metadata from a temp file for simplicity
            fs::metadata("Cargo.toml")
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    fn create_dir_all(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }

    fn remove_file(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }
}

/// Configuration for directory walking operations
#[derive(Debug, Clone, Default)]
pub struct WalkOptions {
    /// File extensions to include (e.g., vec!["rs".to_string(), "py".to_string()])
    pub extensions: Option<Vec<String>>,
    /// Directories to exclude (e.g., vec!["target".to_string(), "node_modules".to_string()])
    pub exclude: Option<Vec<String>>,
    /// Minimum depth (0 = start from root)
    pub min_depth: Option<usize>,
    /// Maximum depth (None = unlimited)
    pub max_depth: Option<usize>,
    /// Whether to follow symbolic links
    pub follow_links: bool,
}

/// Create a filtered WalkDir iterator based on the provided options
///
/// This function eliminates code duplication across the codebase by providing
/// a single, reusable way to create filtered directory walkers.
///
/// # Arguments
///
/// * `path` - Root path to start walking from
/// * `options` - WalkOptions containing filtering criteria
///
/// # Returns
///
/// An iterator that yields only file entries matching the criteria
///
/// # Examples
///
/// ```
/// use codesearch::fs::{create_filtered_walker, WalkOptions};
/// use std::path::Path;
///
/// let options = WalkOptions {
///     extensions: Some(vec!["rs".to_string(), "py".to_string()]),
///     exclude: Some(vec!["target".to_string()]),
///     ..Default::default()
/// };
///
/// let walker = create_filtered_walker(Path::new("./src"), &options);
/// for entry in walker {
///     println!("Found: {:?}", entry.path());
/// }
/// ```
pub fn create_filtered_walker(
    path: &Path,
    options: &WalkOptions,
) -> impl Iterator<Item = walkdir::DirEntry> {
    let mut walk_dir = WalkDir::new(path);

    if let Some(min_depth) = options.min_depth {
        walk_dir = walk_dir.min_depth(min_depth);
    }

    if let Some(max_depth) = options.max_depth {
        walk_dir = walk_dir.max_depth(max_depth);
    }

    if options.follow_links {
        walk_dir = walk_dir.follow_links(true);
    }

    walk_dir
        .into_iter()
        .filter_entry(|e| {
            // Filter out excluded directories
            if let Some(name) = e.file_name().to_str() {
                if let Some(ref exclude_dirs) = options.exclude {
                    for exclude_dir in exclude_dirs {
                        if name == exclude_dir {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
}

/// Get all files matching the given options as a Vec<PathBuf>
///
/// This is a convenience function that collects the walker results into a vector.
///
/// # Arguments
///
/// * `path` - Root path to start walking from
/// * `options` - WalkOptions containing filtering criteria
///
/// # Returns
///
/// A vector of PathBuf objects pointing to matching files
///
/// # Examples
///
/// ```
/// use codesearch::fs::{collect_files, WalkOptions};
/// use std::path::Path;
///
/// let options = WalkOptions {
///     extensions: Some(vec!["rs".to_string()]),
///     ..Default::default()
/// };
///
/// let rust_files = collect_files(Path::new("./src"), &options);
/// println!("Found {} Rust files", rust_files.len());
/// ```
pub fn collect_files(path: &Path, options: &WalkOptions) -> Vec<PathBuf> {
    let walker = create_filtered_walker(path, options);

    walker
        .filter(|entry| {
            // Filter by extension if specified
            if let Some(ref exts) = options.extensions {
                let file_path = entry.path();
                if let Some(ext) = file_path.extension().and_then(|s| s.to_str()) {
                    exts.iter().any(|e| e == ext)
                } else {
                    false
                }
            } else {
                true
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_real_filesystem_read() {
        let fs = RealFileSystem;
        let result = fs.read_to_string(Path::new("Cargo.toml"));
        assert!(result.is_ok());
        assert!(result.unwrap().contains("codesearch"));
    }

    #[test]
    fn test_real_filesystem_exists() {
        let fs = RealFileSystem;
        assert!(fs.exists(Path::new("Cargo.toml")));
        assert!(!fs.exists(Path::new("nonexistent.txt")));
    }

    #[test]
    fn test_mock_filesystem() {
        let mut fs = MockFileSystem::new();
        fs.add_text_file("test.txt", "Hello, World!");

        assert!(fs.exists(Path::new("test.txt")));
        assert!(!fs.exists(Path::new("other.txt")));

        let content = fs.read_to_string(Path::new("test.txt")).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_mock_filesystem_not_found() {
        let fs = MockFileSystem::new();
        let result = fs.read_to_string(Path::new("nonexistent.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_filesystem_multiple_files() {
        let mut fs = MockFileSystem::new();
        fs.add_text_file("file1.txt", "Content 1");
        fs.add_text_file("file2.txt", "Content 2");
        fs.add_text_file("file3.txt", "Content 3");

        assert_eq!(fs.file_count(), 3);
        assert!(fs.has_file(Path::new("file1.txt")));
        assert!(fs.has_file(Path::new("file2.txt")));
        assert!(fs.has_file(Path::new("file3.txt")));
    }

    // Tests for WalkOptions and file walking utilities

    #[test]
    fn test_walk_options_default() {
        let options = WalkOptions::default();
        assert!(options.extensions.is_none());
        assert!(options.exclude.is_none());
        assert!(options.min_depth.is_none());
        assert!(options.max_depth.is_none());
        assert!(!options.follow_links);
    }

    #[test]
    fn test_collect_files_with_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files
        fs::write(dir_path.join("test1.rs"), "fn test1() {}").unwrap();
        fs::write(dir_path.join("test2.rs"), "fn test2() {}").unwrap();
        fs::write(dir_path.join("test.py"), "def test(): pass").unwrap();

        let options = WalkOptions {
            extensions: Some(vec!["rs".to_string()]),
            ..Default::default()
        };

        let files = collect_files(dir_path, &options);
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("test1.rs")));
        assert!(files.iter().any(|p| p.ends_with("test2.rs")));
        assert!(!files.iter().any(|p| p.ends_with("test.py")));
    }

    #[test]
    fn test_collect_files_with_exclude() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create directory structure
        let target_dir = dir_path.join("target");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(dir_path.join("main.rs"), "fn main() {}").unwrap();
        fs::write(target_dir.join("build.rs"), "build script").unwrap();

        let options = WalkOptions {
            exclude: Some(vec!["target".to_string()]),
            ..Default::default()
        };

        let files = collect_files(dir_path, &options);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.rs"));
    }

    #[test]
    fn test_collect_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        let options = WalkOptions::default();
        let files = collect_files(dir_path, &options);

        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_collect_files_multiple_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create test files with different extensions
        fs::write(dir_path.join("test.rs"), "rust code").unwrap();
        fs::write(dir_path.join("test.py"), "python code").unwrap();
        fs::write(dir_path.join("test.js"), "javascript code").unwrap();
        fs::write(dir_path.join("test.txt"), "text file").unwrap();

        let options = WalkOptions {
            extensions: Some(vec!["rs".to_string(), "py".to_string(), "js".to_string()]),
            ..Default::default()
        };

        let files = collect_files(dir_path, &options);
        assert_eq!(files.len(), 3);
        assert!(!files.iter().any(|p| p.ends_with("test.txt")));
    }

    #[test]
    fn test_collect_files_with_max_depth() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Create nested directory structure
        let sub_dir = dir_path.join("src");
        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(dir_path.join("root.rs"), "root").unwrap();
        fs::write(sub_dir.join("nested.rs"), "nested").unwrap();

        let options = WalkOptions {
            max_depth: Some(1), // Only root level
            ..Default::default()
        };

        let files = collect_files(dir_path, &options);
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("root.rs"));
    }
}
