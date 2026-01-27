//! Native parser implementations for multiple programming languages
//!
//! This module provides zero-allocation, Rust-native parsers for various programming languages.
//! Each parser implements the `CodeParser` trait and can extract functions, classes, imports,
//! and variables from source code.

pub mod error;
pub mod token;
pub mod tokenizer;
pub mod traits;
pub mod utils;

pub mod rust;
pub mod python;
pub mod javascript;
pub mod go;
pub mod java;

pub use error::ParseError;
pub use token::{Token, TokenKind};
pub use traits::{CodeParser, ControlFlowParser, ScopeParser};
pub use rust::RustParser;
pub use python::PythonParser;
pub use javascript::JavaScriptParser;
pub use go::GoParser;
pub use java::JavaParser;

// Re-export utility functions for backward compatibility
pub use utils::{
    extract_classes,
    extract_function_calls,
    extract_functions,
    extract_identifier_from_match,
    extract_identifier_references,
    get_file_extension,
    is_keyword_or_builtin,
    read_file_content,
};

/// Get a parser for the given file extension
pub fn get_parser_for_extension(ext: &str) -> Option<Box<dyn CodeParser>> {
    match ext.to_lowercase().as_str() {
        "rs" => Some(Box::new(RustParser)),
        "py" | "pyw" | "pyi" => Some(Box::new(PythonParser)),
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => Some(Box::new(JavaScriptParser)),
        "go" => Some(Box::new(GoParser)),
        "java" => Some(Box::new(JavaParser)),
        _ => None,
    }
}

/// Get a parser for the given language name
pub fn get_parser_for_language(language: &str) -> Option<Box<dyn CodeParser>> {
    match language.to_lowercase().as_str() {
        "rust" => Some(Box::new(RustParser)),
        "python" => Some(Box::new(PythonParser)),
        "javascript" | "typescript" => Some(Box::new(JavaScriptParser)),
        "go" | "golang" => Some(Box::new(GoParser)),
        "java" => Some(Box::new(JavaParser)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parser_rust() {
        let parser = get_parser_for_extension("rs");
        assert!(parser.is_some());
        assert_eq!(parser.unwrap().language_name(), "Rust");
    }

    #[test]
    fn test_get_parser_python() {
        let parser = get_parser_for_extension("py");
        assert!(parser.is_some());
        assert_eq!(parser.unwrap().language_name(), "Python");
    }

    #[test]
    fn test_get_parser_unknown() {
        let parser = get_parser_for_extension("unknown");
        assert!(parser.is_none());
    }
}
