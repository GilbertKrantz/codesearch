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
pub mod cache;
pub mod cache_lru;
pub mod callgraph;
pub mod cfg;
pub mod circular;
#[cfg(test)]
mod circular_tests;
pub mod cli;
pub mod codemetrics;
pub mod commands;
pub mod complexity;
#[cfg(test)]
mod complexity_tests;
pub mod deadcode;
pub mod depgraph;
pub mod designmetrics;
pub mod dfg;
pub mod duplicates;
pub mod errors;
pub mod export;
pub mod extract;
pub mod find;
pub mod fs;
pub mod githistory;
pub mod graphs;
pub mod health;
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
#[cfg(test)]
mod search_tests;
pub mod security;
pub mod traits;
pub mod types;
pub mod unified;
pub mod watcher;

// Re-export commonly used items at the crate root
pub use analysis::analyze_codebase;
pub use ast::{
    AstAnalysis, AstParser, AstSyntaxEdge, ClassInfo, FunctionInfo, SyntaxRelationshipType,
    analyze_file, get_syntax_edges,
};
pub use callgraph::{CallGraph, CallNode, build_call_graph};
pub use cfg::{BasicBlock, ControlFlowGraph, analyze_file_cfg, build_cfg_from_source};
pub use circular::{CircularCall, detect_circular_calls, find_circular_calls};
pub use codemetrics::{
    FileMetrics, ProjectMetrics, analyze_file_metrics, analyze_project_metrics,
    print_metrics_report,
};
pub use complexity::{
    calculate_cognitive_complexity, calculate_cyclomatic_complexity, calculate_file_complexity,
};
pub use deadcode::{DeadCodeItem, detect_dead_code, find_dead_code};
pub use depgraph::{DependencyGraph, DependencyNode, build_dependency_graph};
pub use designmetrics::{
    DesignMetrics, ModuleMetrics, analyze_design_metrics, print_design_metrics,
};
pub use dfg::{DataFlowGraph, DfgNode, analyze_file_dfg, build_dfg_from_source};
pub use duplicates::{detect_duplicates, find_duplicates};
pub use extract::{
    extract_classes, extract_function_calls, extract_functions, extract_identifier_references,
};
pub use find::{FindReport, FindResult, FindType, find_symbol, print_find_report};
pub use githistory::{CommitInfo, GitSearchResult, GitSearcher, search_git_history};
pub use graphs::{GraphAnalysisResult, GraphAnalyzer, GraphType};
pub use health::{HealthReport, print_health_report, scan_health};
pub use index::{CodeIndex, IndexEntry, IndexStats};
pub use language::{LanguageInfo, get_supported_languages};
pub use memopt::{FileReader, StreamingSearcher};
pub use pdg::{ProgramDependencyGraph, analyze_file_pdg, build_pdg_from_source};
pub use remote::{RemoteSearchResult, RemoteSearcher, search_remote_repository};
pub use search::{list_files, print_results, print_search_stats, search_code};
pub use security::{
    SecurityFinding, SecurityKind, Severity, print_security_report, scan_security_patterns,
};
pub use types::{
    ComplexityMetrics, DuplicateBlock, FileInfo, Match, RefactorSuggestion, SearchResult,
};
pub use unified::{
    EdgeCategory, UnifiedEdge, UnifiedGraph, build_unified_graph, trace_data_flow_forward,
    trace_data_flow_in_path,
};
pub use watcher::{FileWatcher, start_watching};

// Re-export fs utilities
pub use fs::{
    FileSystem, MockFileSystem, RealFileSystem, WalkOptions, collect_files, create_filtered_walker,
};
