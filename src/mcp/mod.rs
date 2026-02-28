//! MCP Server Module
//!
//! Provides MCP server functionality for code search operations.
//!
//! This module is organized into sub-modules for better maintainability:
//! - `schemas`: JSON Schema implementations for MCP types
//! - `params`: Parameter structures for MCP tools
//! - `tools`: Tool implementation functions

#[cfg(feature = "mcp")]
mod params;
#[cfg(feature = "mcp")]
mod schemas;
#[cfg(feature = "mcp")]
mod tools;

#[cfg(feature = "mcp")]
pub use params::*;

#[cfg(feature = "mcp")]
use rmcp::{
    ServerHandler,
    handler::server::tool::ToolRouter,
    handler::server::wrapper::{Json, Parameters},
    service::serve_server,
    tool, tool_router,
    transport::io::stdio,
};

#[cfg(feature = "mcp")]
use crate::types::{FileInfo, SearchResult};

#[cfg(feature = "mcp")]
#[derive(Debug, Clone)]
pub struct CodeSearchMcpService {
    tool_router: ToolRouter<Self>,
}

#[cfg(feature = "mcp")]
#[tool_router]
impl CodeSearchMcpService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Search for text patterns in code files with advanced options
    #[tool(
        description = "Search for text patterns in code files with advanced options like fuzzy matching, regex, and filtering"
    )]
    pub async fn search_code(
        &self,
        params: Parameters<SearchCodeParams>,
    ) -> Json<Vec<SearchResult>> {
        tools::search_code_tool(params).await
    }

    /// List all searchable files in a directory
    #[tool(
        description = "List all searchable files in a directory with optional filtering by extensions"
    )]
    pub async fn list_files(&self, params: Parameters<ListFilesParams>) -> Json<Vec<FileInfo>> {
        tools::list_files_tool(params).await
    }

    /// Analyze codebase metrics and statistics
    #[tool(
        description = "Analyze codebase metrics and statistics. Returns JSON with file counts, line counts, and code patterns"
    )]
    pub async fn analyze_codebase(
        &self,
        params: Parameters<AnalyzeCodebaseParams>,
    ) -> Json<serde_json::Value> {
        tools::analyze_codebase_tool(params).await
    }

    /// Detect code complexity issues
    #[tool(
        description = "Detect code complexity issues. Returns files with high cyclomatic or cognitive complexity"
    )]
    pub async fn detect_complexity(
        &self,
        params: Parameters<ComplexityParams>,
    ) -> Json<serde_json::Value> {
        tools::detect_complexity_tool(params).await
    }

    /// Detect duplicate code blocks
    #[tool(description = "Detect duplicate code blocks. Returns pairs of similar code sections")]
    pub async fn detect_duplicates(
        &self,
        params: Parameters<DuplicatesParams>,
    ) -> Json<serde_json::Value> {
        tools::detect_duplicates_tool(params).await
    }

    /// Detect dead code
    #[tool(description = "Detect dead code including unused functions, variables, and imports")]
    pub async fn detect_deadcode(
        &self,
        params: Parameters<DeadcodeParams>,
    ) -> Json<serde_json::Value> {
        tools::detect_deadcode_tool(params).await
    }

    /// Detect circular dependencies
    #[tool(description = "Detect circular dependencies between modules")]
    pub async fn detect_circular(
        &self,
        params: Parameters<CircularParams>,
    ) -> Json<serde_json::Value> {
        tools::detect_circular_tool(params).await
    }

    /// Find symbol: definition, references, callers (structure-aware)
    #[tool(
        description = "Find symbol definition, references, and callers. Structure-aware search for functions, classes, and identifiers"
    )]
    pub async fn find_symbol(
        &self,
        params: Parameters<FindSymbolParams>,
    ) -> Json<serde_json::Value> {
        tools::find_symbol_tool(params).await
    }

    /// Get codebase health score
    #[tool(
        description = "Get codebase health score (0-100) from dead code, duplicates, and complexity. CI-friendly"
    )]
    pub async fn get_health(&self, params: Parameters<GetHealthParams>) -> Json<serde_json::Value> {
        tools::get_health_tool(params).await
    }
}

#[cfg(feature = "mcp")]
impl ServerHandler for CodeSearchMcpService {
    fn get_tool_router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }
}

/// Start the MCP server
#[cfg(feature = "mcp")]
pub async fn start_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let service = CodeSearchMcpService::new();
    let transport = stdio();
    serve_server(service, transport).await?;
    Ok(())
}
