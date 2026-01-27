//! Native Python parser
//!
//! Provides parsing of Python source code with support for indentation-based syntax.

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::traits::CodeParser;
use crate::parser::tokenizer::Tokenizer;

/// Python keywords
const PYTHON_KEYWORDS: &[&str] = &[
    "def", "class", "if", "elif", "else", "for", "while", "try", "except", "finally",
    "with", "as", "import", "from", "return", "yield", "raise", "break", "continue",
    "pass", "lambda", "and", "or", "not", "in", "is", "None", "True", "False",
    "async", "await", "global", "nonlocal", "assert", "del", "assert",
];

/// Native Python parser
pub struct PythonParser;

impl CodeParser for PythonParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, PYTHON_KEYWORDS);
        let tokens = tokenizer.tokenize();

        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut imports = Vec::new();
        let mut variables = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].kind {
                TokenKind::Keyword if tokens[i].text == "def" => {
                    if let Some(func) = self.parse_function(&tokens, i) {
                        functions.push(func);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "class" => {
                    if let Some(class_info) = self.parse_class(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "from" => {
                    if let Some(import_info) = self.parse_from_import(&tokens, i) {
                        let import_line = import_info.line;
                        imports.push(import_info);
                        // Skip ahead to avoid parsing the "import" keyword separately
                        while i < tokens.len() && tokens[i].line == import_line {
                            i += 1;
                        }
                        i -= 1; // Adjust for the increment at the end
                        continue;
                    }
                }
                TokenKind::Keyword if tokens[i].text == "import" => {
                    // Only parse "import" if it's not part of a "from X import Y" statement
                    if i == 0 || tokens[i - 1].text != "from" {
                        if let Some(import_info) = self.parse_import(&tokens, i) {
                            imports.push(import_info);
                        }
                    }
                }
                TokenKind::Identifier if i > 0 && tokens[i - 1].text == "=" => {
                    // Variable assignment
                    if let Some(var_info) = self.parse_variable(&tokens, i) {
                        variables.push(var_info);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Ok(AstAnalysis {
            functions,
            classes,
            imports,
            variables,
        })
    }

    fn extract_functions(&self, content: &str) -> Vec<FunctionInfo> {
        match self.parse_content(content) {
            Ok(analysis) => analysis.functions,
            Err(_) => Vec::new(),
        }
    }

    fn extract_classes(&self, content: &str) -> Vec<ClassInfo> {
        match self.parse_content(content) {
            Ok(analysis) => analysis.classes,
            Err(_) => Vec::new(),
        }
    }

    fn extract_imports(&self, content: &str) -> Vec<ImportInfo> {
        match self.parse_content(content) {
            Ok(analysis) => analysis.imports,
            Err(_) => Vec::new(),
        }
    }

    fn extract_variables(&self, content: &str) -> Vec<VariableInfo> {
        match self.parse_content(content) {
            Ok(analysis) => analysis.variables,
            Err(_) => Vec::new(),
        }
    }

    fn language_name(&self) -> &'static str {
        "Python"
    }

    fn extensions(&self) -> &[&'static str] {
        &["py", "pyw", "pyi"]
    }
}

impl PythonParser {
    /// Parse a function definition
    fn parse_function(&self, tokens: &[Token], start: usize) -> Option<FunctionInfo> {
        let mut is_async = false;

        // Check for async (before 'def')
        if start > 0 && tokens[start - 1].text == "async" {
            is_async = true;
        }

        // Function name (after 'def')
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        // Parse parameters
        let (parameters, pos) = self.parse_parameters(tokens, start + 2)?;

        // Check for return type annotation
        let return_type = if pos < tokens.len() && tokens[pos].text == "->" {
            self.parse_type(tokens, pos + 1)
        } else {
            None
        };

        // Note: Python doesn't have visibility modifiers like Rust/Java
        // All functions are effectively public by default
        // Private functions use underscore prefix convention

        Some(FunctionInfo {
            name,
            line,
            parameters,
            return_type,
            is_async,
            is_public: true, // Python default
        })
    }

    /// Parse function parameters
    fn parse_parameters(&self, tokens: &[Token], start: usize) -> Option<(Vec<String>, usize)> {
        let mut params = Vec::new();
        let mut pos = start;

        // Expect '('
        if pos >= tokens.len() || tokens.get(pos)?.text != "(" {
            return Some((params, pos));
        }
        pos += 1;

        // Parse parameters until ')'
        while pos < tokens.len() {
            if tokens[pos].text == ")" {
                pos += 1;
                break;
            }

            if tokens[pos].text == "," {
                pos += 1;
                continue;
            }

            // Extract parameter name (identifier before ':' or '=' or ',')
            if tokens[pos].kind == TokenKind::Identifier {
                let param_name = tokens[pos].text.to_string();
                params.push(param_name);
                pos += 1;

                // Skip type annotation or default value
                if pos < tokens.len() {
                    if tokens[pos].text == ":" || tokens[pos].text == "=" {
                        pos += 1;
                        // Skip until comma or closing paren
                        while pos < tokens.len()
                            && tokens[pos].text != ","
                            && tokens[pos].text != ")"
                        {
                            pos += 1;
                        }
                    }
                }
            } else {
                pos += 1;
            }
        }

        Some((params, pos))
    }

    /// Parse a type annotation
    fn parse_type(&self, tokens: &[Token], start: usize) -> Option<String> {
        let mut type_str = String::new();
        let mut pos = start;
        let mut bracket_depth = 0;

        while pos < tokens.len() {
            match tokens[pos].text {
                "[" | "(" => bracket_depth += 1,
                "]" | ")" => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                    } else {
                        break;
                    }
                }
                ":" | "=" | "," => {
                    if bracket_depth == 0 {
                        break;
                    }
                }
                _ => {}
            }

            if !type_str.is_empty() {
                type_str.push(' ');
            }
            type_str.push_str(tokens[pos].text);
            pos += 1;
        }

        if type_str.is_empty() {
            None
        } else {
            Some(type_str.trim().to_string())
        }
    }

    /// Parse a class definition
    fn parse_class(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        // Class name (after 'class')
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        // Check for inheritance
        let mut pos = start + 2;
        if pos < tokens.len() && tokens[pos].text == "(" {
            // Skip inheritance list
            pos += 1;
            let mut paren_depth = 1;
            while pos < tokens.len() && paren_depth > 0 {
                if tokens[pos].text == "(" {
                    paren_depth += 1;
                } else if tokens[pos].text == ")" {
                    paren_depth -= 1;
                }
                pos += 1;
            }
        }

        // Look for ':' (end of class declaration)
        if pos < tokens.len() && tokens[pos].text == ":" {
            pos += 1;
        }

        // Note: We're not parsing the class body here since we'd need to track indentation
        // For now, return what we have
        Some(ClassInfo {
            name,
            line,
            methods: Vec::new(),
            fields: Vec::new(),
            is_public: true, // Python default
        })
    }

    /// Parse an import statement
    fn parse_import(&self, tokens: &[Token], start: usize) -> Option<ImportInfo> {
        let mut module = String::from("import");
        let mut items = Vec::new();
        let mut pos = start + 1; // Skip 'import'
        let start_line = tokens[start].line;

        // Parse module path (stay on the same line)
        while pos < tokens.len() && tokens[pos].line == start_line {
            if tokens[pos].kind == TokenKind::Identifier || tokens[pos].text == "." {
                module.push(' ');
                module.push_str(tokens[pos].text);
            }
            pos += 1;
        }

        // The module name itself is the imported item for "import X"
        let module_parts: Vec<&str> = module.split_whitespace().collect();
        if !module_parts.is_empty() {
            let main_import = module_parts.last()?.to_string();
            items.push(main_import);
        }

        Some(ImportInfo {
            module: module.trim().to_string(),
            line: tokens[start].line,
            items,
        })
    }

    /// Parse a "from X import Y" statement
    fn parse_from_import(&self, tokens: &[Token], start: usize) -> Option<ImportInfo> {
        let mut module = String::new();
        let mut items = Vec::new();
        let mut pos = start + 1; // Skip 'from'
        let start_line = tokens[start].line;

        // Parse module path (until 'import')
        while pos < tokens.len() && tokens[pos].text != "import" {
            if tokens[pos].kind == TokenKind::Identifier || tokens[pos].text == "." {
                module.push_str(tokens[pos].text);
            }
            pos += 1;
        }

        // Skip 'import'
        pos += 1;

        // Parse imported items (stay on the same line)
        while pos < tokens.len() && tokens[pos].line == start_line {
            if tokens[pos].kind == TokenKind::Identifier {
                items.push(tokens[pos].text.to_string());
            }
            pos += 1;
        }

        Some(ImportInfo {
            module: module.trim().to_string(),
            line: tokens[start].line,
            items,
        })
    }

    /// Parse a variable declaration
    fn parse_variable(&self, tokens: &[Token], start: usize) -> Option<VariableInfo> {
        // Variable name
        let name = tokens[start].text.to_string();
        let line = tokens[start].line;

        // Check if it's const (all caps naming convention)
        let is_const = name.chars().all(|c| c.is_uppercase() || c == '_');
        let is_mutable = !is_const; // Python variables are mutable by default

        Some(VariableInfo {
            name,
            line,
            is_const,
            is_mutable,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function_simple() {
        let parser = PythonParser;
        let code = r#"
            def hello():
                print("Hello")
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "hello");
        assert!(!functions[0].is_async);
    }

    #[test]
    fn test_parse_function_async() {
        let parser = PythonParser;
        let code = r#"
            async def fetch_data():
                return await api_call()
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "fetch_data");
        assert!(functions[0].is_async);
    }

    #[test]
    fn test_parse_function_with_params() {
        let parser = PythonParser;
        let code = r#"
            def add(x: int, y: int) -> int:
                return x + y
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "add");
        assert_eq!(functions[0].parameters, vec!["x", "y"]);
        assert_eq!(functions[0].return_type, Some("int".to_string()));
    }

    #[test]
    fn test_parse_class() {
        let parser = PythonParser;
        let code = r#"
            class Point:
                def __init__(self, x, y):
                    self.x = x
                    self.y = y
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
    }

    #[test]
    fn test_parse_class_inheritance() {
        let parser = PythonParser;
        let code = r#"
            class Animal:
                pass

            class Dog(Animal):
                pass
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].name, "Animal");
        assert_eq!(classes[1].name, "Dog");
    }

    #[test]
    fn test_parse_import() {
        let parser = PythonParser;
        let code = r#"
            import os
            import sys
            import numpy as np
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 3);
        assert_eq!(imports[0].module, "import os");
        assert_eq!(imports[1].module, "import sys");
    }

    #[test]
    fn test_parse_from_import() {
        let parser = PythonParser;
        let code = r#"
            from collections import defaultdict
            from typing import List, Dict, Optional
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module, "collections");
        assert_eq!(imports[0].items, vec!["defaultdict"]);
        assert_eq!(imports[1].module, "typing");
        assert_eq!(imports[1].items, vec!["List", "Dict", "Optional"]);
    }
}
