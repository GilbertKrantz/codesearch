//! Token types and definitions for native parsers

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Keywords
    Keyword,

    // Identifiers
    Identifier,

    // Literals
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    BooleanLiteral,

    // Operators
    Operator,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Dot,
    DoubleDot,
    TripleDot,
    At,
    Pound,
    Dollar,
    Question,

    // Whitespace and comments
    Whitespace,
    LineComment,
    BlockComment,

    // Other
    Unknown,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Keyword => write!(f, "keyword"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::StringLiteral => write!(f, "string literal"),
            TokenKind::CharLiteral => write!(f, "character literal"),
            TokenKind::NumberLiteral => write!(f, "number literal"),
            TokenKind::BooleanLiteral => write!(f, "boolean literal"),
            TokenKind::Operator => write!(f, "operator"),
            TokenKind::LeftParen => write!(f, "'('"),
            TokenKind::RightParen => write!(f, "')'"),
            TokenKind::LeftBrace => write!(f, "'{{'"),
            TokenKind::RightBrace => write!(f, "'}}'"),
            TokenKind::LeftBracket => write!(f, "'['"),
            TokenKind::RightBracket => write!(f, "']'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Semicolon => write!(f, "';'"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::DoubleDot => write!(f, "'..'"),
            TokenKind::TripleDot => write!(f, "'...'"),
            TokenKind::At => write!(f, "'@'"),
            TokenKind::Pound => write!(f, "'#'"),
            TokenKind::Dollar => write!(f, "'$'"),
            TokenKind::Question => write!(f, "'?'"),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::LineComment => write!(f, "line comment"),
            TokenKind::BlockComment => write!(f, "block comment"),
            TokenKind::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub line: usize,
    pub column: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, text: &'a str, line: usize, column: usize) -> Self {
        Self {
            kind,
            text,
            line,
            column,
        }
    }

    pub fn is_keyword(&self, keyword: &str) -> bool {
        self.kind == TokenKind::Keyword && self.text == keyword
    }

    pub fn is_operator(&self, op: &str) -> bool {
        self.kind == TokenKind::Operator && self.text == op
    }

    pub fn is_identifier(&self) -> bool {
        self.kind == TokenKind::Identifier
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} '{}' at {}:{}",
            self.kind, self.text, self.line, self.column
        )
    }
}
