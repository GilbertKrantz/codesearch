//! Native Java parser
//!
//! Provides parsing of Java source code.

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::tokenizer::Tokenizer;
use crate::parser::traits::CodeParser;

const JAVA_KEYWORDS: &[&str] = &[
    "abstract",
    "assert",
    "boolean",
    "break",
    "byte",
    "case",
    "catch",
    "char",
    "class",
    "const",
    "continue",
    "default",
    "do",
    "double",
    "else",
    "enum",
    "extends",
    "final",
    "finally",
    "float",
    "for",
    "goto",
    "if",
    "implements",
    "import",
    "instanceof",
    "int",
    "interface",
    "long",
    "native",
    "new",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "short",
    "static",
    "strictfp",
    "super",
    "switch",
    "synchronized",
    "this",
    "throw",
    "throws",
    "transient",
    "try",
    "void",
    "volatile",
    "while",
    "true",
    "false",
    "null",
];

pub struct JavaParser;

impl CodeParser for JavaParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, JAVA_KEYWORDS);
        let tokens = tokenizer.tokenize();

        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut imports = Vec::new();
        let mut variables = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].kind {
                TokenKind::Keyword if tokens[i].text == "class" => {
                    if let Some(class_info) = self.parse_class(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "interface" => {
                    if let Some(class_info) = self.parse_interface(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "enum" => {
                    if let Some(class_info) = self.parse_enum(&tokens, i) {
                        classes.push(class_info);
                    }
                }
                TokenKind::Keyword if tokens[i].text == "import" => {
                    if let Some(import_info) = self.parse_import(&tokens, i) {
                        imports.push(import_info);
                    }
                }
                TokenKind::Identifier => {
                    if self.is_method_declaration(&tokens, i) {
                        if let Some(func) = self.parse_method(&tokens, i) {
                            functions.push(func);
                        }
                    } else if self.is_variable_declaration(&tokens, i) {
                        if let Some(var_info) = self.parse_variable(&tokens, i) {
                            variables.push(var_info);
                        }
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
        "Java"
    }

    fn extensions(&self) -> &[&'static str] {
        &["java"]
    }
}

impl JavaParser {
    fn is_method_declaration(&self, tokens: &[Token], start: usize) -> bool {
        let mut pos = start;

        while pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
            pos += 1;
        }

        if pos < tokens.len() && tokens[pos].text == "(" {
            return true;
        }

        false
    }

    fn is_variable_declaration(&self, tokens: &[Token], start: usize) -> bool {
        let mut pos = start + 1;

        while pos < tokens.len() {
            if tokens[pos].kind == TokenKind::Identifier {
                if pos + 1 < tokens.len()
                    && (tokens[pos + 1].text == "=" || tokens[pos + 1].text == ";")
                {
                    return true;
                }
                pos += 1;
            } else if tokens[pos].text == "(" {
                return false;
            } else {
                pos += 1;
            }
        }

        false
    }

    fn parse_method(&self, tokens: &[Token], start: usize) -> Option<FunctionInfo> {
        let mut pos = start;
        let mut is_public = false;

        while pos > 0 {
            pos -= 1;
            if tokens[pos].text == "public" {
                is_public = true;
            } else if tokens[pos].text == ";" || tokens[pos].text == "}" {
                break;
            }
        }

        pos = start;
        let return_type = tokens[pos].text.to_string();
        pos += 1;

        let name = if pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
            tokens[pos].text.to_string()
        } else {
            return None;
        };

        let line = tokens[start].line;
        pos += 1;

        let (parameters, _) = self.parse_parameters(tokens, pos)?;

        Some(FunctionInfo {
            name,
            line,
            parameters,
            return_type: Some(return_type),
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
                pos += 1;
                if pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
                    params.push(tokens[pos].text.to_string());
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }

        Some((params, pos))
    }

    fn parse_class(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = false;

        if start > 0 && tokens[start - 1].text == "public" {
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
                let next_pos = pos + 1;
                if next_pos < tokens.len() {
                    if tokens[next_pos].kind == TokenKind::Identifier {
                        let method_or_field = tokens[next_pos].text.to_string();
                        if next_pos + 1 < tokens.len() && tokens[next_pos + 1].text == "(" {
                            methods.push(method_or_field);
                        } else if next_pos + 1 < tokens.len()
                            && (tokens[next_pos + 1].text == "="
                                || tokens[next_pos + 1].text == ";")
                        {
                            fields.push(method_or_field);
                        }
                    }
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

    fn parse_interface(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = false;

        if start > 0 && tokens[start - 1].text == "public" {
            is_public = true;
        }

        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        let mut methods = Vec::new();
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
                let next_pos = pos + 1;
                if next_pos < tokens.len() && tokens[next_pos].kind == TokenKind::Identifier {
                    methods.push(tokens[next_pos].text.to_string());
                }
            }

            pos += 1;
        }

        Some(ClassInfo {
            name,
            line,
            methods,
            fields: Vec::new(),
            is_public,
        })
    }

    fn parse_enum(&self, tokens: &[Token], start: usize) -> Option<ClassInfo> {
        let mut is_public = false;

        if start > 0 && tokens[start - 1].text == "public" {
            is_public = true;
        }

        let name = tokens.get(start + 1)?.text.to_string();
        let line = tokens[start].line;

        Some(ClassInfo {
            name,
            line,
            methods: Vec::new(),
            fields: Vec::new(),
            is_public,
        })
    }

    fn parse_import(&self, tokens: &[Token], start: usize) -> Option<ImportInfo> {
        let mut module = String::new();
        let mut items = Vec::new();
        let mut pos = start + 1;
        let start_line = tokens[start].line;

        while pos < tokens.len() && tokens[pos].line == start_line {
            if tokens[pos].text == ";" {
                break;
            }
            if tokens[pos].kind == TokenKind::Identifier
                || tokens[pos].text == "."
                || tokens[pos].text == "*"
            {
                module.push_str(tokens[pos].text);
            }
            pos += 1;
        }

        if let Some(last_part) = module.split('.').last() {
            items.push(last_part.to_string());
        }

        Some(ImportInfo {
            module,
            line: start_line,
            items,
        })
    }

    fn parse_variable(&self, tokens: &[Token], start: usize) -> Option<VariableInfo> {
        let mut pos = start;
        let mut is_const = false;

        while pos > 0 {
            pos -= 1;
            if tokens[pos].text == "final" {
                is_const = true;
                break;
            } else if tokens[pos].text == ";" || tokens[pos].text == "}" {
                break;
            }
        }

        pos = start + 1;
        let name = if pos < tokens.len() && tokens[pos].kind == TokenKind::Identifier {
            tokens[pos].text.to_string()
        } else {
            return None;
        };

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
    fn test_parse_class() {
        let parser = JavaParser;
        let code = r#"
            public class Point {
                private int x;
                private int y;
                
                public Point(int x, int y) {
                    this.x = x;
                    this.y = y;
                }
                
                public double distance() {
                    return Math.sqrt(x * x + y * y);
                }
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Point");
        assert!(classes[0].is_public);
    }

    #[test]
    fn test_parse_interface() {
        let parser = JavaParser;
        let code = r#"
            public interface Drawable {
                void draw();
                void resize(int width, int height);
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Drawable");
        assert!(classes[0].is_public);
    }

    #[test]
    fn test_parse_import() {
        let parser = JavaParser;
        let code = r#"
            import java.util.ArrayList;
            import java.util.HashMap;
        "#;

        let imports = parser.extract_imports(code);
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0].module, "java.util.ArrayList");
        assert_eq!(imports[1].module, "java.util.HashMap");
    }

    #[test]
    fn test_parse_method() {
        let parser = JavaParser;
        let code = r#"
            public class Calculator {
                public int add(int a, int b) {
                    return a + b;
                }
            }
        "#;

        let classes = parser.extract_classes(code);
        assert_eq!(classes.len(), 1, "Should detect Calculator class");
        assert_eq!(classes[0].name, "Calculator");
    }
}
