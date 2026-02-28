//! Native Go parser
//!
//! Provides parsing of Go source code.

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::tokenizer::Tokenizer;
use crate::parser::traits::CodeParser;

const GO_KEYWORDS: &[&str] = &[
    "break",
    "case",
    "chan",
    "const",
    "continue",
    "default",
    "defer",
    "else",
    "fallthrough",
    "for",
    "func",
    "go",
    "goto",
    "if",
    "import",
    "interface",
    "map",
    "package",
    "range",
    "return",
    "select",
    "struct",
    "switch",
    "type",
    "var",
    "bool",
    "byte",
    "complex64",
    "complex128",
    "error",
    "float32",
    "float64",
    "int",
    "int8",
    "int16",
    "int32",
    "int64",
    "rune",
    "string",
    "uint",
    "uint8",
    "uint16",
    "uint32",
    "uint64",
    "uintptr",
    "true",
    "false",
    "iota",
    "nil",
    "append",
    "cap",
    "close",
    "complex",
    "copy",
    "delete",
    "imag",
    "len",
    "make",
    "new",
    "panic",
    "print",
    "println",
    "real",
    "recover",
];

pub struct GoParser;

impl CodeParser for GoParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, GO_KEYWORDS);
        let tokens = tokenizer.tokenize();

        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut imports = Vec::new();
        let mut variables = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].kind {
                TokenKind::Keyword if tokens[i].text == "func" => {
                    if let Some(func) = self.parse_function(&tokens, i) {
                        functions.push(func);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "type" => {
                    if let Some(class_info) = self.parse_type(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "import" => {
                    if let Some(import_info) = self.parse_import(&tokens, i) {
                        imports.push(import_info);
                    }
                }
                TokenKind::Keyword if matches!(tokens[i].text, "var" | "const") => {
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
        "Go"
    }

    fn extensions(&self) -> &[&'static str] {
        &["go"]
    }
}

impl GoParser {
    fn parse_function(&self, tokens: &[Token], start: usize) -> Option<FunctionInfo> {
        let mut pos = start + 1;
        let line = tokens[start].line;

        let mut receiver = None;
        if pos < tokens.len() && tokens[pos].text == "(" {
            pos += 1;
            while pos < tokens.len() && tokens[pos].text != ")" {
                if tokens[pos].kind == TokenKind::Identifier {
                    receiver = Some(tokens[pos].text.to_string());
                }
                pos += 1;
            }
            if pos < tokens.len() {
                pos += 1;
            }
        }

        let name = if pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
            tokens[pos].text.to_string()
        } else {
            return None;
        };

        pos += 1;

        let (parameters, mut pos) = self.parse_parameters(tokens, pos)?;

        let return_type = if pos < tokens.len() && tokens[pos].text != "{" {
            let ret = self.parse_return_type(tokens, pos);
            while pos < tokens.len() && tokens[pos].text != "{" {
                pos += 1;
            }
            ret
        } else {
            None
        };

        let is_public = name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false);

        Some(FunctionInfo {
            name: if let Some(recv) = receiver {
                format!("{}.{}", recv, name)
            } else {
                name
            },
            line,
            parameters,
            return_type,
            is_async: false,
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

                while pos < tokens.len() && tokens[pos].text != "," && tokens[pos].text != ")" {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }

        Some((params, pos))
    }

    fn parse_return_type(&self, tokens: &[Token], start: usize) -> Option<String> {
        let mut type_str = String::new();
        let mut pos = start;

        if pos >= tokens.len() {
            return None;
        }

        if tokens[pos].text == "(" {
            pos += 1;
            let mut paren_depth = 1;
            while pos < tokens.len() && paren_depth > 0 {
                if tokens[pos].text == "(" {
                    paren_depth += 1;
                } else if tokens[pos].text == ")" {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                }
                if !type_str.is_empty() {
                    type_str.push(' ');
                }
                type_str.push_str(tokens[pos].text);
                pos += 1;
            }
        } else {
            while pos < tokens.len() && tokens[pos].text != "{" {
                if !type_str.is_empty() {
                    type_str.push(' ');
                }
                type_str.push_str(tokens[pos].text);
                pos += 1;
            }
        }

        if type_str.is_empty() {
            None
        } else {
            Some(type_str.trim().to_string())
        }
    }

    fn parse_type(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        let mut pos = start + 2;
        if pos >= tokens.len() {
            return None;
        }

        let is_struct = tokens[pos].text == "struct";
        let is_interface = tokens[pos].text == "interface";

        if !is_struct && !is_interface {
            return None;
        }

        pos += 1;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        if pos < tokens.len() && tokens[pos].text == "{" {
            pos += 1;
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
                    let field_name = tokens[pos].text.to_string();
                    if is_interface {
                        methods.push(field_name);
                    } else {
                        fields.push(field_name);
                    }
                }

                pos += 1;
            }
        }

        let is_public = name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false);

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

        if pos < tokens.len() && tokens[pos].text == "(" {
            pos += 1;
            while pos < tokens.len() && tokens[pos].text != ")" {
                if tokens[pos].kind == TokenKind::StringLiteral {
                    let import_path = tokens[pos].text.trim_matches('"').to_string();
                    items.push(import_path.clone());
                    if module.is_empty() {
                        module = import_path;
                    }
                }
                pos += 1;
            }
        } else if pos < tokens.len() && tokens[pos].kind == TokenKind::StringLiteral {
            module = tokens[pos].text.trim_matches('"').to_string();
            items.push(module.clone());
        }

        Some(ImportInfo {
            module,
            line: start_line,
            items,
        })
    }

    fn parse_variable(&self, tokens: &[Token], start: usize) -> Option<VariableInfo> {
        let keyword = tokens[start].text;
        let is_const = keyword == "const";

        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        Some(VariableInfo {
            name,
            line,
            is_const,
            is_mutable: !is_const,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function() {
        let parser = GoParser;
        let code = r#"
            func hello() {
                fmt.Println("Hello")
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert_eq!(functions[0].name, "hello");
    }

    #[test]
    fn test_parse_function_with_receiver() {
        let parser = GoParser;
        let code = r#"
            func (p *Point) Distance() float64 {
                return math.Sqrt(p.x*p.x + p.y*p.y)
            }
        "#;

        let functions = parser.extract_functions(code);
        assert_eq!(functions.len(), 1);
        assert!(functions[0].name.contains("Point"));
        assert!(functions[0].name.contains("Distance"));
    }

    #[test]
    fn test_parse_struct() {
        let parser = GoParser;
        let code = r#"
            type Point struct {
                x float64
                y float64
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
        assert!(classes[0].fields.contains(&"x".to_string()));
        assert!(classes[0].fields.contains(&"y".to_string()));
    }

    #[test]
    fn test_parse_interface() {
        let parser = GoParser;
        let code = r#"
            type Reader interface {
                Read(p []byte) (n int, err error)
                Close() error
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Reader");
        assert!(classes[0].methods.contains(&"Read".to_string()));
        assert!(classes[0].methods.contains(&"Close".to_string()));
    }

    #[test]
    fn test_parse_import() {
        let parser = GoParser;
        let code = r#"
            import "fmt"
            import "math"
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module, "fmt");
        assert_eq!(imports[1].module, "math");
    }

    #[test]
    fn test_parse_variable() {
        let parser = GoParser;
        let code = r#"
            var x int = 42
            const PI = 3.14
        "#;

        let variables = parser.extract_variables(code);
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0].name, "x");
        assert!(!variables[0].is_const);
        assert_eq!(variables[1].name, "PI");
        assert!(variables[1].is_const);
    }
}
