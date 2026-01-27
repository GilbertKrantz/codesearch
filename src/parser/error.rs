//! Error types for native parsers

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Syntax error at a specific location
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    /// Unexpected token encountered
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
    },

    /// End of input reached unexpectedly
    UnexpectedEof {
        expected: String,
        line: usize,
    },

    /// Unsupported language feature
    UnsupportedFeature(String),

    /// Invalid input
    InvalidInput(String),

    /// IO error wrapper
    IoError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SyntaxError { line, column, message } => {
                write!(f, "Syntax error at line {}, column {}: {}", line, column, message)
            }
            ParseError::UnexpectedToken { expected, found, line } => {
                write!(f, "Unexpected token at line {}: expected '{}', found '{}'", line, expected, found)
            }
            ParseError::UnexpectedEof { expected, line } => {
                write!(f, "Unexpected end of file at line {}: expected '{}'", line, expected)
            }
            ParseError::UnsupportedFeature(feature) => {
                write!(f, "Unsupported language feature: {}", feature)
            }
            ParseError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
            ParseError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err.to_string())
    }
}
