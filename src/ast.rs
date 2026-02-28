//! AST-Based Code Analysis Module
//!
//! Provides syntax tree analysis for precise code structure understanding using native parsers.
//!
//! ## Syntax relationship edges
//!
//! Use [`get_syntax_edges`] to extract explicit syntax relationships (parent-child, contains, imports)
//! from an [`AstAnalysis`]. These edges complement execution flow (CFG) and data dependencies (DFG).

use crate::parser::{ParseError, get_parser_for_extension};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Syntax relationship type for AST edges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyntaxRelationshipType {
    /// Parent contains child (module/class scope)
    Contains,
    /// Function has parameter
    HasParameter,
    /// Class has method
    HasMethod,
    /// Class has field
    HasField,
    /// Import references module
    Imports,
}

/// An edge representing a syntax relationship between two AST elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstSyntaxEdge {
    pub from_node: String,
    pub from_line: usize,
    pub to_node: String,
    pub to_line: usize,
    pub relationship: SyntaxRelationshipType,
}

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
            Err(ParseError::UnsupportedFeature(format!(
                "Extension '{}' not supported",
                ext
            )))
        }
    }

    pub fn parse_file(&mut self, path: &Path) -> Result<AstAnalysis, ParseError> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let parser = get_parser_for_extension(ext).ok_or_else(|| {
            ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext))
        })?;

        parser.parse_file(path)
    }

    pub fn parse_content(&mut self, content: &str, ext: &str) -> Result<AstAnalysis, ParseError> {
        let parser = get_parser_for_extension(ext).ok_or_else(|| {
            ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext))
        })?;

        parser.parse_content(content)
    }
}

/// Extract syntax relationship edges from AST analysis.
///
/// Returns explicit edges for: class→method, class→field, function→parameter, import→module.
/// Together with CFG (execution flow) and DFG (data dependencies), these cover all relationship types.
pub fn get_syntax_edges(analysis: &AstAnalysis) -> Vec<AstSyntaxEdge> {
    let mut edges = Vec::new();

    for func in &analysis.functions {
        for param in &func.parameters {
            edges.push(AstSyntaxEdge {
                from_node: func.name.clone(),
                from_line: func.line,
                to_node: param.clone(),
                to_line: func.line,
                relationship: SyntaxRelationshipType::HasParameter,
            });
        }
    }

    for class in &analysis.classes {
        for method in &class.methods {
            edges.push(AstSyntaxEdge {
                from_node: class.name.clone(),
                from_line: class.line,
                to_node: method.clone(),
                to_line: class.line,
                relationship: SyntaxRelationshipType::HasMethod,
            });
        }
        for field in &class.fields {
            edges.push(AstSyntaxEdge {
                from_node: class.name.clone(),
                from_line: class.line,
                to_node: field.clone(),
                to_line: class.line,
                relationship: SyntaxRelationshipType::HasField,
            });
        }
    }

    for import in &analysis.imports {
        edges.push(AstSyntaxEdge {
            from_node: import.module.clone(),
            from_line: import.line,
            to_node: import.items.join(", "),
            to_line: import.line,
            relationship: SyntaxRelationshipType::Imports,
        });
    }

    edges
}

/// Analyze a file and extract AST information
pub fn analyze_file(path: &Path) -> Result<AstAnalysis, ParseError> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let parser = get_parser_for_extension(ext).ok_or_else(|| {
        ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext))
    })?;

    parser.parse_file(path)
}

/// Analyze source code content and extract AST information
pub fn analyze_content(content: &str, ext: &str) -> Result<AstAnalysis, ParseError> {
    let parser = get_parser_for_extension(ext).ok_or_else(|| {
        ParseError::UnsupportedFeature(format!("Extension '{}' not supported", ext))
    })?;

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

    #[test]
    fn test_get_syntax_edges() {
        let code = r#"
            fn add(x: i32, y: i32) -> i32 { x + y }
            struct Point { x: f64, y: f64 }
        "#;
        let analysis = analyze_content(code, "rs").unwrap();
        let edges = get_syntax_edges(&analysis);
        assert!(!edges.is_empty());
        let has_param = edges
            .iter()
            .any(|e| matches!(e.relationship, SyntaxRelationshipType::HasParameter));
        let has_field = edges
            .iter()
            .any(|e| matches!(e.relationship, SyntaxRelationshipType::HasField));
        assert!(has_param || has_field);
    }
}
