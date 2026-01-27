//! AST-Based Code Analysis Module
//!
//! Provides syntax tree analysis for precise code structure understanding using native parsers.

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::parser::{CodeParser, get_parser_for_extension, ParseError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstNode {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub children: Vec<AstNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAnalysis {
    pub functions: Vec<FunctionInfo>,
    pub classes: Vec<ClassInfo>,
    pub imports: Vec<ImportInfo>,
    pub variables: Vec<VariableInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub name: String,
    pub line: usize,
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    pub module: String,
    pub line: usize,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub line: usize,
    pub is_const: bool,
    pub is_mutable: bool,
}

/// Legacy AstParser struct for backward compatibility
pub struct AstParser;

impl AstParser {
    pub fn new_rust() -> Result<Self, ParseError> {
        Ok(Self)
    }

    pub fn new_python() -> Result<Self, ParseError> {
        Ok(Self)
    }

    pub fn new_javascript() -> Result<Self, ParseError> {
        Ok(Self)
    }

    pub fn for_extension(ext: &str) -> Result<Self, ParseError> {
        if get_parser_for_extension(ext).is_some() {
            Ok(Self)
        } else {
            Err(ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext)))
        }
    }

    pub fn parse_file(&mut self, path: &Path) -> Result<AstAnalysis, ParseError> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let parser = get_parser_for_extension(ext)
            .ok_or_else(|| ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext)))?;

        parser.parse_file(path)
    }

    pub fn parse_content(&mut self, content: &str, ext: &str) -> Result<AstAnalysis, ParseError> {
        let parser = get_parser_for_extension(ext)
            .ok_or_else(|| ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext)))?;

        parser.parse_content(content)
    }
}

/// Analyze a file and extract AST information
pub fn analyze_file(path: &Path) -> Result<AstAnalysis, ParseError> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let parser = get_parser_for_extension(ext)
        .ok_or_else(|| ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext)))?;

    parser.parse_file(path)
}

/// Analyze source code content and extract AST information
pub fn analyze_content(content: &str, ext: &str) -> Result<AstAnalysis, ParseError> {
    let parser = get_parser_for_extension(ext)
        .ok_or_else(|| ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext)))?;

    parser.parse_content(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_rust_code() {
        let code = r#"
            fn hello() {
                println!("Hello");
            }

            pub struct Point {
                x: i32,
                y: i32,
            }
        "#;

        let result = analyze_content(code, "rs");
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert_eq!(analysis.functions.len(), 1);
        assert_eq!(analysis.functions[0].name, "hello");
        assert_eq!(analysis.classes.len(), 1);
        assert_eq!(analysis.classes[0].name, "Point");
    }

    #[test]
    fn test_analyze_python_code() {
        let code = r#"
            def hello():
                print("Hello")

            class Point:
                pass
        "#;

        let result = analyze_content(code, "py");
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert_eq!(analysis.functions.len(), 1);
        assert_eq!(analysis.functions[0].name, "hello");
        assert_eq!(analysis.classes.len(), 1);
        assert_eq!(analysis.classes[0].name, "Point");
    }
}
