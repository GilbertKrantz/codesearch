//! Generic tokenizer for programming languages
//!
//! Provides a base tokenizer implementation that can be extended for specific languages.

use crate::parser::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

/// Base tokenizer that can be configured for different languages
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    source: &'a str,
    position: usize,
    line: usize,
    column: usize,
    keywords: &'a [&'static str],
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str, keywords: &'a [&'static str]) -> Self {
        Self {
            input: source.chars().peekable(),
            source,
            position: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            // Skip whitespace tokens but keep comments
            if token.kind != TokenKind::Whitespace {
                tokens.push(token);
            }
        }

        tokens
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Option<Token<'a>> {
        // Skip whitespace first
        self.skip_whitespace();

        let c = self.peek()?;

        let start_line = self.line;
        let start_column = self.column;

        match c {
            // EOF
            _ if c == '\0' => None,

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => Some(self.scan_identifier()),

            // Numbers
            '0'..='9' => Some(self.scan_number()),

            // String literals
            '"' => Some(self.scan_string()),

            // Character literals
            '\'' => Some(self.scan_char()),

            // Operators (excluding '/' which is handled below for comments)
            '+' | '-' | '*' | '%' | '&' | '|' | '^' | '!' | '=' | '<' | '>' => {
                Some(self.scan_operator())
            }

            // Delimiters
            '(' => {
                self.advance();
                Some(Token::new(
                    TokenKind::LeftParen,
                    "(",
                    start_line,
                    start_column,
                ))
            }
            ')' => {
                self.advance();
                Some(Token::new(
                    TokenKind::RightParen,
                    ")",
                    start_line,
                    start_column,
                ))
            }
            '{' => {
                self.advance();
                Some(Token::new(
                    TokenKind::LeftBrace,
                    "{",
                    start_line,
                    start_column,
                ))
            }
            '}' => {
                self.advance();
                Some(Token::new(
                    TokenKind::RightBrace,
                    "}",
                    start_line,
                    start_column,
                ))
            }
            '[' => {
                self.advance();
                Some(Token::new(
                    TokenKind::LeftBracket,
                    "[",
                    start_line,
                    start_column,
                ))
            }
            ']' => {
                self.advance();
                Some(Token::new(
                    TokenKind::RightBracket,
                    "]",
                    start_line,
                    start_column,
                ))
            }
            ',' => {
                self.advance();
                Some(Token::new(TokenKind::Comma, ",", start_line, start_column))
            }
            ':' => {
                self.advance();
                Some(Token::new(TokenKind::Colon, ":", start_line, start_column))
            }
            ';' => {
                self.advance();
                Some(Token::new(
                    TokenKind::Semicolon,
                    ";",
                    start_line,
                    start_column,
                ))
            }
            '.' => {
                self.advance();
                if self.peek() == Some('.') {
                    self.advance();
                    if self.peek() == Some('.') {
                        self.advance();
                        Some(Token::new(
                            TokenKind::TripleDot,
                            "...",
                            start_line,
                            start_column,
                        ))
                    } else {
                        Some(Token::new(
                            TokenKind::DoubleDot,
                            "..",
                            start_line,
                            start_column,
                        ))
                    }
                } else {
                    Some(Token::new(TokenKind::Dot, ".", start_line, start_column))
                }
            }
            '@' => {
                self.advance();
                Some(Token::new(TokenKind::At, "@", start_line, start_column))
            }
            '#' => {
                self.advance();
                Some(Token::new(TokenKind::Pound, "#", start_line, start_column))
            }
            '$' => {
                self.advance();
                Some(Token::new(TokenKind::Dollar, "$", start_line, start_column))
            }
            '?' => {
                self.advance();
                Some(Token::new(
                    TokenKind::Question,
                    "?",
                    start_line,
                    start_column,
                ))
            }

            // Comments (language-specific, default to C-style)
            '/' => Some(self.scan_comment_or_operator()),

            // Unknown
            _ => {
                self.advance();
                Some(Token::new(
                    TokenKind::Unknown,
                    &self.source[self.position - 1..self.position],
                    start_line,
                    start_column,
                ))
            }
        }
    }

    fn scan_identifier(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // First character (already validated)
        self.advance();

        // Continue while alphanumeric or underscore
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.position];
        let kind = if self.keywords.contains(&text) {
            TokenKind::Keyword
        } else {
            TokenKind::Identifier
        };

        Token::new(kind, text, start_line, start_column)
    }

    fn scan_number(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // Integer part
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part
        if self.peek() == Some('.') {
            self.advance();
            while let Some(&c) = self.input.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Exponent part
        if self.peek() == Some('e') || self.peek() == Some('E') {
            self.advance();
            if self.peek() == Some('+') || self.peek() == Some('-') {
                self.advance();
            }
            while let Some(&c) = self.input.peek() {
                if c.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let text = &self.source[start..self.position];
        Token::new(TokenKind::NumberLiteral, text, start_line, start_column)
    }

    fn scan_string(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // Opening quote
        self.advance();

        // Content until closing quote
        while let Some(&c) = self.input.peek() {
            if c == '\\' {
                // Escape sequence
                self.advance();
                self.advance();
            } else if c == '"' {
                self.advance();
                break;
            } else {
                self.advance();
            }
        }

        let text = &self.source[start..self.position];
        Token::new(TokenKind::StringLiteral, text, start_line, start_column)
    }

    fn scan_char(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // Opening quote
        self.advance();

        // Content
        if let Some(&c) = self.input.peek() {
            if c == '\\' {
                // Escape sequence
                self.advance();
            }
            self.advance();
        }

        // Closing quote
        if self.peek() == Some('\'') {
            self.advance();
        }

        let text = &self.source[start..self.position];
        Token::new(TokenKind::CharLiteral, text, start_line, start_column)
    }

    fn scan_operator(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // First character
        self.advance();

        // Check for multi-character operators
        match self.peek() {
            Some('=') | Some('&') | Some('|') | Some('>') | Some('<') => {
                self.advance();
            }
            _ => {}
        }

        let text = &self.source[start..self.position];
        Token::new(TokenKind::Operator, text, start_line, start_column)
    }

    fn scan_comment_or_operator(&mut self) -> Token<'a> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        // First slash
        self.advance();

        match self.peek() {
            Some('/') => {
                // Line comment
                self.advance();
                while let Some(&c) = self.input.peek() {
                    if c != '\n' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let text = &self.source[start..self.position];
                Token::new(TokenKind::LineComment, text, start_line, start_column)
            }
            Some('*') => {
                // Block comment
                self.advance();
                while let Some(&c) = self.input.peek() {
                    self.advance();
                    if c == '*' && self.peek() == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                }
                let text = &self.source[start..self.position];
                Token::new(TokenKind::BlockComment, text, start_line, start_column)
            }
            _ => {
                // Division operator
                let text = &self.source[start..self.position];
                Token::new(TokenKind::Operator, text, start_line, start_column)
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.advance();
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
            } else {
                break;
            }
        }
    }

    // Helper methods
    fn advance(&mut self) {
        self.input.next();
        self.position += 1;
        self.column += 1;
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_keywords() {
        let keywords = &["fn", "let", "mut", "pub", "struct", "impl"];
        let mut tokenizer = Tokenizer::new("fn let mut", keywords);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 3);
        assert!(tokens[0].is_keyword("fn"));
        assert!(tokens[1].is_keyword("let"));
        assert!(tokens[2].is_keyword("mut"));
    }

    #[test]
    fn test_tokenize_identifiers() {
        let keywords = &["fn"];
        let mut tokenizer = Tokenizer::new("foo bar baz", keywords);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 3);
        assert!(tokens[0].is_identifier());
        assert_eq!(tokens[0].text, "foo");
    }

    #[test]
    fn test_tokenize_numbers() {
        let keywords = &[];
        let mut tokenizer = Tokenizer::new("123 45.67 1e10", keywords);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[0].text, "123");
        assert_eq!(tokens[1].text, "45.67");
        assert_eq!(tokens[2].text, "1e10");
    }

    #[test]
    fn test_tokenize_strings() {
        let keywords = &[];
        let mut tokenizer = Tokenizer::new(r#""hello" "world""#, keywords);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].text, r#""hello""#);
    }

    #[test]
    fn test_tokenize_delimiters() {
        let keywords = &[];
        let mut tokenizer = Tokenizer::new("(){}[],;:", keywords);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::LeftParen);
        assert_eq!(tokens[1].kind, TokenKind::RightParen);
        assert_eq!(tokens[2].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[3].kind, TokenKind::RightBrace);
        assert_eq!(tokens[4].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[5].kind, TokenKind::RightBracket);
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        assert_eq!(tokens[7].kind, TokenKind::Semicolon);
        assert_eq!(tokens[8].kind, TokenKind::Colon);
    }

    #[test]
    fn test_tokenize_line_comments() {
        let keywords = &["fn"];
        let mut tokenizer = Tokenizer::new("// comment\nfn", keywords);
        let tokens = tokenizer.tokenize();

        // Line comment + "fn" keyword
        assert!(tokens.len() >= 2);
        assert_eq!(tokens[0].kind, TokenKind::LineComment);
        assert!(tokens[1].is_keyword("fn"));
    }
}
