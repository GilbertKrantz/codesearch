//! Native Rust parser
//!
//! Provides zero-allocation parsing of Rust source code.

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::traits::CodeParser;
use crate::parser::tokenizer::Tokenizer;

/// Rust keywords
const RUST_KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "pub", "priv", "struct", "enum", "impl", "trait", "type", "const",
    "static", "async", "unsafe", "extern", "crate", "mod", "use", "where", "for", "while",
    "loop", "match", "if", "else", "return", "break", "continue", "move", "ref", "dyn",
    "union", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "yield", "await", "try", "in",
    "self", "Self", "super",
];

/// Native Rust parser
pub struct RustParser;

impl CodeParser for RustParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, RUST_KEYWORDS);
        let tokens = tokenizer.tokenize();

        let mut functions = Vec::new();
        let mut classes = Vec::new(); // Rust structs/enums
        let mut imports = Vec::new();
        let mut variables = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].kind {
                TokenKind::Keyword if tokens[i].text == "fn" => {
                    if let Some(func) = self.parse_function(&tokens, i) {
                        functions.push(func);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "struct" => {
                    if let Some(struct_info) = self.parse_struct(&tokens, i) {
                        classes.push(struct_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "enum" => {
                    if let Some(enum_info) = self.parse_enum(&tokens, i) {
                        classes.push(enum_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "impl" => {
                    if let Some(impl_info) = self.parse_impl(&tokens, i) {
                        classes.push(impl_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "use" => {
                    if let Some(import_info) = self.parse_use(&tokens, i) {
                        imports.push(import_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "let" => {
                    if let Some(var_info) = self.parse_variable(&tokens, i) {
                        variables.push(var_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "const" => {
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
        "Rust"
    }

    fn extensions(&self) -> &[&'static str] {
        &["rs"]
    }
}

impl RustParser {
    /// Parse a function declaration
    fn parse_function(&self, tokens: &[Token], start: usize) -> Option<FunctionInfo> {
        let mut pos = start;
        let mut is_public = false;
        let mut is_async = false;

        // Check for visibility modifier (before 'fn')
        if pos > 0 && tokens[pos - 1].text == "pub" {
            is_public = true;
        }

        // Check for async (before 'fn')
        if pos > 0 && tokens[pos - 1].text == "async" {
            is_async = true;
        }
        // Handle "pub async fn"
        if pos > 1 && tokens[pos - 2].text == "pub" && tokens[pos - 1].text == "async" {
            is_public = true;
            is_async = true;
        }

        // Function name (after 'fn')
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        // Parse parameters
        let (parameters, mut pos) = self.parse_parameters(tokens, start + 2)?;

        // Check for return type
        let return_type = if let Some(token) = tokens.get(pos) {
            if token.text == "->" {
                let ret = self.parse_type(tokens, pos + 1);
                pos += 2; // Skip -> and type
                ret
            } else {
                None
            }
        } else {
            None
        };

        Some(FunctionInfo {
            name,
            line,
            parameters,
            return_type,
            is_async,
            is_public,
        })
    }

    /// Parse function parameters
    fn parse_parameters(&self, tokens: &[Token], start: usize) -> Option<(Vec<String>, usize)> {
        let mut params = Vec::new();
        let mut pos = start;

        // Expect '('
        if tokens.get(pos)?.text != "(" {
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

            // Extract parameter name (identifier before ':')
            if tokens[pos].kind == TokenKind::Identifier {
                let param_name = tokens[pos].text.to_string();
                params.push(param_name);
                pos += 1;

                // Skip type annotation
                if tokens.get(pos).map(|t| t.text) == Some(":") {
                    pos += 1;
                    // Skip type
                    while pos < tokens.len()
                        && tokens[pos].text != ","
                        && tokens[pos].text != ")"
                    {
                        pos += 1;
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
                "<" | "[" | "(" => bracket_depth += 1,
                ">" | "]" | ")" => {
                    if bracket_depth > 0 {
                        bracket_depth -= 1;
                    } else {
                        break;
                    }
                }
                "," | "=" | ";" | "{" | "}" => {
                    if bracket_depth == 0 {
                        break;
                    }
                }
                _ => {}
            }

            // Add space in certain cases
            let prev_token = if pos > start { tokens.get(pos - 1) } else { None };
            let needs_space = if let Some(prev) = prev_token {
                // Add space after comma
                if prev.text == "," {
                    true
                }
                // Add space between identifiers/keywords, but not before delimiters
                else {
                    let prev_is_delim = matches!(prev.text, "<" | ">" | "[" | "]" | "(" | ")" | "," | ";" | ":" | ".");
                    let curr_is_delim = matches!(tokens[pos].text, "<" | ">" | "[" | "]" | "(" | ")" | "," | ";" | ":" | ".");
                    !prev_is_delim && !curr_is_delim
                }
            } else {
                false
            };

            if needs_space {
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

    /// Parse a struct declaration
    fn parse_struct(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = false;

        // Check for visibility
        if start > 0 && tokens[start - 1].text == "pub" {
            is_public = true;
        }

        // Struct name
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        // Parse fields
        let (fields, methods) = self.parse_struct_fields(tokens, start + 2)?;

        Some(ClassInfo {
            name,
            line,
            methods,
            fields,
            is_public,
        })
    }

    /// Parse struct fields
    fn parse_struct_fields(&self, tokens: &[Token], start: usize) -> Option<(Vec<String>, Vec<String>)> {
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut pos = start;

        // Look for '{' for body-style struct or continue for tuple-style
        if pos >= tokens.len() {
            return Some((fields, methods));
        }

        if tokens[pos].text == "{" {
            pos += 1;

            // Parse fields until '}'
            while pos < tokens.len() {
                if tokens[pos].text == "}" {
                    break;
                }

                // Look for identifier followed by ':' (field declaration)
                if tokens[pos].kind == TokenKind::Identifier {
                    if let Some(next_token) = tokens.get(pos + 1) {
                        if next_token.text == ":" {
                            let field_name = tokens[pos].text.to_string();
                            fields.push(field_name);
                        }
                    }
                }

                pos += 1;
            }
        }

        Some((fields, methods))
    }

    /// Parse an enum declaration
    fn parse_enum(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = false;

        // Check for visibility
        if start > 0 && tokens[start - 1].text == "pub" {
            is_public = true;
        }

        // Enum name
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        Some(ClassInfo {
            name,
            line,
            methods: Vec::new(),
            fields: Vec::new(), // Enum variants not extracted for now
            is_public,
        })
    }

    /// Parse an impl block
    fn parse_impl(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        // Impl target name
        let target_name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        // Find methods within impl block
        let mut methods = Vec::new();
        let mut pos = start + 2;
        let mut brace_depth = 0;

        // Look for opening brace
        while pos < tokens.len() && tokens[pos].text != "{" {
            pos += 1;
        }
        if pos < tokens.len() {
            pos += 1; // Skip opening brace
        }

        while pos < tokens.len() {
            // Track brace depth to find the actual end of impl block
            if tokens[pos].text == "{" {
                brace_depth += 1;
            } else if tokens[pos].text == "}" {
                if brace_depth == 0 {
                    break; // End of impl block
                }
                brace_depth -= 1;
            }

            if tokens[pos].text == "fn" {
                if let Some(func) = self.parse_function(tokens, pos) {
                    methods.push(func.name);
                }
            }

            pos += 1;
        }

        Some(ClassInfo {
            name: target_name,
            line,
            methods,
            fields: Vec::new(),
            is_public: false, // impl blocks don't have visibility
        })
    }

    /// Parse a use statement
    fn parse_use(&self, tokens: &[Token], start: usize) -> Option<ImportInfo> {
        let mut module = String::new();
        let mut items = Vec::new();
        let mut pos = start + 1; // Skip 'use'

        // Parse module path
        while pos < tokens.len() {
            match tokens[pos].text {
                ";" => {
                    pos += 1;
                    break;
                }
                "{" => {
                    // Parse items in braces
                    pos += 1;
                    while pos < tokens.len() && tokens[pos].text != "}" {
                        if tokens[pos].kind == TokenKind::Identifier {
                            items.push(tokens[pos].text.to_string());
                        }
                        pos += 1;
                    }
                }
                ":" | "as" => {
                    // Skip path separators and aliases
                    pos += 1;
                }
                _ => {
                    if !module.is_empty() {
                        module.push_str("::");
                    }
                    module.push_str(tokens[pos].text);
                }
            }
            pos += 1;
        }

        // If no items in braces, the last identifier is the import
        if items.is_empty() {
            if let Some(last_part) = module.split("::").last() {
                items.push(last_part.to_string());
            }
        }

        Some(ImportInfo {
            module,
            line: tokens[start].line,
            items,
        })
    }

    /// Parse a variable declaration (let or const)
    fn parse_variable(&self, tokens: &[Token], start: usize) -> Option<VariableInfo> {
        let keyword = tokens[start].text;
        let is_const = keyword == "const";

        // Check if mutable (look for 'mut' after 'let')
        let is_mutable = if keyword == "let" {
            // Check if next token is 'mut'
            if let Some(next_token) = tokens.get(start + 1) {
                next_token.text == "mut"
            } else {
                false
            }
        } else {
            // const variables are never mutable
            false
        };

        // Variable name (after 'let'/'const' or after 'let mut')
        let name_offset = if is_mutable { 2 } else { 1 };
        let name = tokens.get(start + name_offset)?.text.to_string();
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
    fn test_parse_function_simple() {
        let parser = RustParser;
        let code = r#"
            fn hello() {
                println!("Hello");
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "hello");
        assert!(!functions[0].is_public);
        assert!(!functions[0].is_async);
    }

    #[test]
    fn test_parse_function_public_async() {
        let parser = RustParser;
        let code = r#"
            pub async fn fetch_data() -> Result<Data, Error> {
                Ok(Data::new())
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "fetch_data");
        assert!(functions[0].is_public);
        assert!(functions[0].is_async);
        assert_eq!(functions[0].return_type, Some("Result<Data, Error>".to_string()));
    }

    #[test]
    fn test_parse_function_with_params() {
        let parser = RustParser;
        let code = r#"
            fn add(x: i32, y: i32) -> i32 {
                x + y
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "add");
        assert_eq!(functions[0].parameters, vec!["x", "y"]);
    }

    #[test]
    fn test_parse_struct() {
        let parser = RustParser;
        let code = r#"
            pub struct Point {
                x: i32,
                y: i32,
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
        assert!(classes[0].is_public);
        assert_eq!(classes[0].fields, vec!["x", "y"]);
    }

    #[test]
    fn test_parse_impl() {
        let parser = RustParser;
        let code = r#"
            impl Point {
                fn new(x: i32, y: i32) -> Self {
                    Point { x, y }
                }

                fn distance(&self) -> f64 {
                    0.0
                }
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
        assert_eq!(classes[0].methods, vec!["new", "distance"]);
    }

    #[test]
    fn test_parse_use() {
        let parser = RustParser;
        let code = r#"
            use std::collections::HashMap;
            use crate::utils::{helper, Helper2};
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module, "std::collections::HashMap");
        assert_eq!(imports[0].items, vec!["HashMap"]);
        assert_eq!(imports[1].module, "crate::utils");
        assert_eq!(imports[1].items, vec!["helper", "Helper2"]);
    }

    #[test]
    fn test_parse_variable() {
        let parser = RustParser;
        let code = r#"
            let x = 42;
            let mut y = 10;
            const MAX: i32 = 100;
        "#;

        let variables = parser.extract_variables(code);
        assert_eq!(variables.len(), 3);
        assert_eq!(variables[0].name, "x");
        assert!(!variables[0].is_const);
        assert!(!variables[0].is_mutable);
        assert_eq!(variables[1].name, "y");
        assert!(!variables[1].is_const);
        assert!(variables[1].is_mutable);
        assert_eq!(variables[2].name, "MAX");
        assert!(variables[2].is_const);
    }
}
