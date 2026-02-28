//! CodeSearch - A fast CLI tool for searching codebases
//!
//! This library provides code search functionality with support for:
//! - Regex and fuzzy search
//! - Multi-language support
//! - Codebase analysis
//! - Complexity metrics
//! - Duplicate detection
//! - Dead code detection
//! - Interactive mode
//! - MCP server integration

pub mod analysis;
pub mod ast;
pub mod extract;
pub mod cache;
pub mod callgraph;
pub mod cfg;
pub mod cli;
pub mod commands;
pub mod codemetrics;
pub mod circular;
#[cfg(test)]
mod circular_tests;
pub mod complexity;
#[cfg(test)]
mod complexity_tests;
pub mod deadcode;
pub mod depgraph;
pub mod designmetrics;
pub mod dfg;
pub mod duplicates;
pub mod cache_lru;
pub mod errors;
pub mod export;
pub mod find;
pub mod fs;
pub mod githistory;
pub mod health;
pub mod graphs;
pub mod index;
pub mod interactive;
pub mod language;
#[cfg(feature = "mcp")]
pub mod mcp;
pub mod memopt;
pub mod parser;
pub mod pdg;
pub mod remote;
pub mod search;
pub mod traits;
#[cfg(test)]
mod search_tests;
pub mod types;
pub mod watcher;

// Re-export commonly used items at the crate root
pub use search::{list_files, print_results, print_search_stats, search_code};
pub use types::{ComplexityMetrics, DuplicateBlock, FileInfo, Match, RefactorSuggestion, SearchResult};
pub use analysis::analyze_codebase;
pub use ast::{
    analyze_file, get_syntax_edges, AstAnalysis, AstParser, AstSyntaxEdge, ClassInfo, FunctionInfo,
    SyntaxRelationshipType,
};
pub use callgraph::{build_call_graph, CallGraph, CallNode};
pub use cfg::{analyze_file_cfg, build_cfg_from_source, ControlFlowGraph, BasicBlock};
pub use codemetrics::{analyze_file_metrics, analyze_project_metrics, print_metrics_report, FileMetrics, ProjectMetrics};
pub use complexity::{calculate_file_complexity, calculate_cyclomatic_complexity, calculate_cognitive_complexity};
pub use circular::{detect_circular_calls, find_circular_calls, CircularCall};
pub use deadcode::{detect_dead_code, find_dead_code, DeadCodeItem};
pub use extract::{extract_classes, extract_function_calls, extract_functions, extract_identifier_references};
pub use depgraph::{build_dependency_graph, DependencyGraph, DependencyNode};
pub use designmetrics::{analyze_design_metrics, print_design_metrics, DesignMetrics, ModuleMetrics};
pub use dfg::{analyze_file_dfg, build_dfg_from_source, DataFlowGraph, DfgNode};
pub use duplicates::{detect_duplicates, find_duplicates};
pub use find::{find_symbol, FindReport, FindResult, FindType, print_find_report};
pub use githistory::{search_git_history, GitSearcher, GitSearchResult, CommitInfo};
pub use health::{print_health_report, scan_health, HealthReport};
pub use graphs::{GraphAnalyzer, GraphAnalysisResult, GraphType};
pub use index::{CodeIndex, IndexEntry, IndexStats};
pub use language::{get_supported_languages, LanguageInfo};
pub use memopt::{FileReader, StreamingSearcher};
pub use pdg::{analyze_file_pdg, build_pdg_from_source, ProgramDependencyGraph};
pub use remote::{search_remote_repository, RemoteSearcher, RemoteSearchResult};
pub use watcher::{start_watching, FileWatcher};

// Re-export fs utilities
pub use fs::{FileSystem, RealFileSystem, MockFileSystem, WalkOptions, create_filtered_walker, collect_files};

