//! Native JavaScript/TypeScript parser
//!
//! Provides parsing of JavaScript and TypeScript source code.

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::traits::CodeParser;
use crate::parser::tokenizer::Tokenizer;

const JS_KEYWORDS: &[&str] = &[
    "function", "async", "await", "class", "extends", "constructor",
    "const", "let", "var", "if", "else", "for", "while", "do", "switch",
    "case", "break", "continue", "return", "throw", "try", "catch", "finally",
    "new", "this", "super", "static", "public", "private", "protected",
    "import", "export", "from", "default", "as", "typeof", "instanceof",
    "void", "delete", "in", "of", "yield", "interface", "type", "enum",
    "namespace", "module", "declare", "abstract", "implements", "readonly",
];

pub struct JavaScriptParser;

impl CodeParser for JavaScriptParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, JS_KEYWORDS);
        let tokens = tokenizer.tokenize();

        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut imports = Vec::new();
        let mut variables = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].kind {
                TokenKind::Keyword if tokens[i].text == "function" => {
                    if let Some(func) = self.parse_function(&tokens, i) {
                        functions.push(func);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "class" => {
                    if let Some(class_info) = self.parse_class(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "import" => {
                    if let Some(import_info) = self.parse_import(&tokens, i) {
                        imports.push(import_info);
                    }
                }
                TokenKind::Keyword if matches!(tokens[i].text, "const" | "let" | "var") => {
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
        "JavaScript"
    }

    fn extensions(&self) -> &[&'static str] {
        &["js", "jsx", "ts", "tsx", "mjs", "cjs"]
    }
}

impl JavaScriptParser {
    fn parse_function(&self, tokens: &[Token], start: usize) -> Option<FunctionInfo> {
        let mut pos = start;
        let mut is_async = false;
        let mut is_public = true;

        if pos > 0 && tokens[pos - 1].text == "async" {
            is_async = true;
        }
        if pos > 0 && tokens[pos - 1].text == "export" {
            is_public = true;
        }

        pos += 1;

        let name = if pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
            tokens[pos].text.to_string()
        } else {
            "anonymous".to_string()
        };

        let line = tokens[start].line;

        let (parameters, _) = if pos + 1 < tokens.len() {
            self.parse_parameters(tokens, pos + 1).unwrap_or((Vec::new(), pos + 1))
        } else {
            (Vec::new(), pos + 1)
        };

        Some(FunctionInfo {
            name,
            line,
            parameters,
            return_type: None,
            is_async,
            is_public,
        })
    }

    fn parse_parameters(&self, tokens: &[Token], start: usize) -> Option<(Vec<String>, usize)> {
        let mut params = Vec::new();
        let mut pos = start;

        if pos >= tokens.len() || tokens[pos].text != "(" {
            return Some((params, pos));
        }
        pos += 1;

        while pos < tokens.len() {
            if tokens[pos].text == ")" {
                pos += 1;
                break;
            }

            if tokens[pos].text == "," {
                pos += 1;
                continue;
            }

            if tokens[pos].kind == TokenKind::Identifier {
                params.push(tokens[pos].text.to_string());
                pos += 1;

                if pos < tokens.len() && (tokens[pos].text == ":" || tokens[pos].text == "=") {
                    pos += 1;
                    while pos < tokens.len() && tokens[pos].text != "," && tokens[pos].text != ")" {
                        pos += 1;
                    }
                }
            } else {
                pos += 1;
            }
        }

        Some((params, pos))
    }

    fn parse_class(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = true;

        if start > 0 && tokens[start - 1].text == "export" {
            is_public = true;
        }

        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        let mut methods = Vec::new();
        let mut fields = Vec::new();
        let mut pos = start + 2;

        while pos < tokens.len() && tokens[pos].text != "{" {
            pos += 1;
        }
        if pos < tokens.len() {
            pos += 1;
        }

        let mut brace_depth = 1;
        while pos < tokens.len() && brace_depth > 0 {
            if tokens[pos].text == "{" {
                brace_depth += 1;
            } else if tokens[pos].text == "}" {
                brace_depth -= 1;
                if brace_depth == 0 {
                    break;
                }
            }

            if brace_depth == 1 && tokens[pos].kind == TokenKind::Identifier {
                let method_name = tokens[pos].text.to_string();
                let next_pos = pos + 1;
                
                if next_pos < tokens.len() && tokens[next_pos].text == "(" {
                    methods.push(method_name);
                } else if next_pos < tokens.len() && (tokens[next_pos].text == "=" || tokens[next_pos].text == ";") {
                    fields.push(method_name);
                }
            }

            pos += 1;
        }

        Some(ClassInfo {
            name,
            line,
            methods,
            fields,
            is_public,
        })
    }

    fn parse_import(&self, tokens: &[Token], start: usize) -> Option<ImportInfo> {
        let mut module = String::new();
        let mut items = Vec::new();
        let mut pos = start + 1;
        let start_line = tokens[start].line;
        let mut in_braces = false;

        while pos < tokens.len() && tokens[pos].line == start_line {
            if tokens[pos].text == "from" {
                pos += 1;
                while pos < tokens.len() && tokens[pos].line == start_line {
                    if tokens[pos].kind == TokenKind::StringLiteral {
                        let raw = tokens[pos].text;
                        module = raw.trim_matches(|c: char| c == '"' || c == '\'' || c.is_whitespace()).to_string();
                        break;
                    }
                    pos += 1;
                }
                break;
            } else if tokens[pos].text == "{" {
                in_braces = true;
            } else if tokens[pos].text == "}" {
                in_braces = false;
            } else if in_braces && tokens[pos].kind == TokenKind::Identifier {
                items.push(tokens[pos].text.to_string());
            } else if !in_braces && tokens[pos].kind == TokenKind::Identifier && tokens[pos].text != "as" {
                items.push(tokens[pos].text.to_string());
            } else if tokens[pos].kind == TokenKind::StringLiteral && module.is_empty() {
                let raw = tokens[pos].text;
                module = raw.trim_matches(|c: char| c == '"' || c == '\'' || c.is_whitespace()).to_string();
            }
            pos += 1;
        }

        Some(ImportInfo {
            module,
            line: tokens[start].line,
            items,
        })
    }

    fn parse_variable(&self, tokens: &[Token], start: usize) -> Option<VariableInfo> {
        let keyword = tokens[start].text;
        let is_const = keyword == "const";
        let is_mutable = keyword != "const";

        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

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
    fn test_parse_function() {
        let parser = JavaScriptParser;
        let code = r#"
            function hello() {
                console.log("Hello");
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "hello");
    }

    #[test]
    fn test_parse_async_function() {
        let parser = JavaScriptParser;
        let code = r#"
            async function fetchData() {
                return await fetch('/api');
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "fetchData");
        assert!(functions[0].is_async);
    }

    #[test]
    fn test_parse_class() {
        let parser = JavaScriptParser;
        let code = r#"
            class Point {
                constructor(x, y) {
                    this.x = x;
                    this.y = y;
                }
                
                distance() {
                    return Math.sqrt(this.x * this.x + this.y * this.y);
                }
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
        assert!(classes[0].methods.contains(&"distance".to_string()));
    }

    #[test]
    fn test_parse_import() {
        let parser = JavaScriptParser;
        let code = r#"
            import { useState, useEffect } from 'react';
            import axios from 'axios';
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 2, "Should detect 2 import statements");
        assert!(imports[0].items.contains(&"useState".to_string()));
        assert!(imports[0].items.contains(&"useEffect".to_string()));
    }

    #[test]
    fn test_parse_variable() {
        let parser = JavaScriptParser;
        let code = r#"
            const x = 42;
            let y = 10;
            var z = 5;
        "#;

        let variables = parser.extract_variables(code);
        assert_eq!(variables.len(), 3);
        assert_eq!(variables[0].name, "x");
        assert!(variables[0].is_const);
        assert_eq!(variables[1].name, "y");
        assert!(variables[1].is_mutable);
    }
}
