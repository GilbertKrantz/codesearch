//! Parser trait definitions and common types

use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use std::path::Path;

/// Core parser trait that all language parsers must implement
pub trait CodeParser: Send + Sync {
    /// Parse a file and return AST analysis
    fn parse_file(&self, path: &Path) -> Result<AstAnalysis, ParseError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| ParseError::IoError(e.to_string()))?;
        self.parse_content(&content)
    }

    /// Parse source code content and return AST analysis
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError>;

    /// Extract functions from source code
    fn extract_functions(&self, content: &str) -> Vec<FunctionInfo>;

    /// Extract classes/structs from source code
    fn extract_classes(&self, content: &str) -> Vec<ClassInfo>;

    /// Extract imports from source code
    fn extract_imports(&self, content: &str) -> Vec<ImportInfo>;

    /// Extract variables from source code
    fn extract_variables(&self, content: &str) -> Vec<VariableInfo>;

    /// Get the language name this parser handles
    fn language_name(&self) -> &'static str;

    /// Get file extensions this parser supports
    fn extensions(&self) -> &[&'static str];
}

/// Trait for extracting control flow information
pub trait ControlFlowParser {
    /// Extract basic blocks for CFG building
    fn extract_basic_blocks(&self, content: &str) -> Result<Vec<BasicBlock>, ParseError>;

    /// Extract if/else branches
    fn extract_branches(&self, content: &str) -> Result<Vec<Branch>, ParseError>;

    /// Extract loops
    fn extract_loops(&self, content: &str) -> Result<Vec<Loop>, ParseError>;
}

/// Trait for extracting scope information
pub trait ScopeParser {
    /// Extract variable scopes
    fn extract_scopes(&self, content: &str) -> Result<Vec<Scope>, ParseError>;

    /// Find all references to a variable
    fn find_variable_references(
        &self,
        content: &str,
        var_name: &str,
    ) -> Result<Vec<Reference>, ParseError>;
}

/// Basic block in control flow graph
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub kind: BlockKind,
    pub statements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockKind {
    Entry,
    Exit,
    Normal,
    Branch,
    Loop,
    Return,
}

/// Branch in control flow (if/else, match)
#[derive(Debug, Clone)]
pub struct Branch {
    pub condition: String,
    pub line: usize,
    pub true_block: Option<usize>,
    pub false_block: Option<usize>,
}

/// Loop in control flow
#[derive(Debug, Clone)]
pub struct Loop {
    pub loop_type: LoopType,
    pub line: usize,
    pub body_block: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    While,
    Loop,
    For,
    ForIn,
}

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: usize,
    pub start_line: usize,
    pub end_line: usize,
    pub parent_id: Option<usize>,
    pub variables: Vec<String>,
}

/// Reference to a variable
#[derive(Debug, Clone)]
pub struct Reference {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub is_definition: bool,
    pub is_usage: bool,
}
