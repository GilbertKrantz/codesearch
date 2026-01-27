# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CodeSearch is a fast, intelligent CLI tool written in Rust (edition 2024) that provides comprehensive code search and analysis for 48+ programming languages. It serves as both a replacement for traditional grep and a sophisticated code analysis platform with dead code detection, complexity metrics, duplicate detection, and graph analysis capabilities.

## Build and Development Commands

### Building
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# With MCP server support (AI integration)
cargo build --features mcp --release
```

### Testing
```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Common Development Tasks
```bash
# Check code without building
cargo check

# Run linter (clippy)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Run the tool
cargo run -- codesearch "query"
cargo run --release -- codesearch "pattern" ./src

# Run in interactive mode
cargo run -- interactive

# Start MCP server for AI integration
cargo run --features mcp -- mcp-server
```

## Architecture Overview

### Module Structure (40+ modules)

The codebase follows a modular architecture with clear separation of concerns:

#### Core Systems
- **`search/`** - Search engine with regex, fuzzy matching, and parallel processing
  - Sub-modules: `core`, `engine`, `fuzzy`, `semantic`, `pure`, `utilities`
  - Entry point: `search_code(query, path, &SearchOptions)`
  - Key feature: Parallel file processing with rayon

- **`deadcode/`** - Modularized dead code detection system
  - Sub-modules: `types`, `helpers`, `detectors`
  - Detects: unused variables, unreachable code, empty functions, TODO markers, commented code, unused imports
  - Pattern: Two-pass algorithm (collect definitions → find low usage)

- **`analysis/`** - Codebase metrics and refactoring suggestions
- **`complexity/`** - Cyclomatic and cognitive complexity metrics
- **`duplicates/`** - Code clone detection (Type-1/2/3 clones)

#### Graph Analysis (6 types)
- **`ast/`** - Abstract Syntax Tree (code structure)
- **`cfg/`** - Control Flow Graph (execution paths)
- **`dfg/`** - Data Flow Graph (variable dependencies)
- **`callgraph/`** - Call Graph (function relationships)
- **`depgraph/`** - Dependency Graph (module dependencies)
- **`pdg/`** - Program Dependency Graph (combined analysis)

#### Supporting Modules
- **`language/`** - 48+ language definitions with syntax patterns
- **`parser/`** - Code parsing utilities
- **`cli/`** - CLI interface using clap
- **`commands/`** - Command handlers and routing
- **`export/`** - CSV/Markdown export functionality
- **`cache/`**, **`cache_lru/`** - Thread-safe caching with DashMap
- **`index/`** - Incremental indexing for large codebases
- **`watcher/`** - Real-time file monitoring
- **`githistory/`** - Git history search
- **`remote/`** - GitHub/GitLab repository search
- **`codemetrics/`** - Halstead and other code metrics
- **`designmetrics/`** - Coupling, cohesion, instability analysis
- **`circular/`** - Circular dependency detection
- **`memopt/`** - Memory optimization utilities
- **`mcp/`** - Model Context Protocol server for AI agents

### Data Flow Pattern

Most commands follow this flow:
1. **CLI** (`main.rs`) parses command
2. **Command Handler** (in `commands/`) validates and routes
3. **Core Module** (e.g., `search/`, `deadcode/`) processes logic
4. **File System** (via `fs/` or `walkdir`) reads files
5. **Parallel Processing** (via `rayon`) handles multiple files
6. **Results** formatted and returned to user

### Key Design Patterns

1. **Module Pattern with Sub-modules**: Complex modules like `deadcode/`, `search/`, `duplicates/` are split into focused sub-modules following Separation of Concerns

2. **Two-Pass Detection**: Used in `deadcode/` and other analysis modules
   - Pass 1: Collect all definitions/references
   - Pass 2: Analyze and flag issues

3. **Parallel Processing**: Most file operations use `rayon` for parallel execution
   ```rust
   files.par_iter().filter_map(|file| process_file(file))
   ```

4. **Thread-Safe Caching**: Uses `DashMap` for concurrent access
   ```rust
   let cache = DashMap::new();
   cache.insert(key, value);
   ```

5. **Options Struct Pattern**: Complex functions use builder-style options
   ```rust
   let options = SearchOptions {
       extensions: Some(vec!["rs".to_string()]),
       fuzzy: true,
       ..Default::default()
   };
   search_code(query, path, &options);
   ```

## Configuration

### Configuration Files (`.codesearchrc` or `.codesearch.toml`)
Configuration files are checked in this order:
1. `.codesearchrc` (current directory)
2. `.codesearch.toml` (current directory)
3. `~/.codesearchrc` (home directory)
4. `~/.codesearch.toml` (home directory)

Example configuration:
```toml
[search]
fuzzy_threshold = 0.6
max_results = 10
ignore_case = true
show_line_numbers = true
format = "text"
cache = false
semantic = false
rank = false
```

### Language Support
The tool supports 48+ languages defined in `src/language/mod.rs`. Language patterns are used for:
- Function/class extraction
- Syntax highlighting
- Dead code detection (brace-based vs indentation-based)

## Testing Strategy

### Test Organization
- **Unit Tests**: Co-located with implementation code in `#[cfg(test)]` modules
- **Integration Tests**: Located in `tests/` directory
- **Property-Based Tests**: Use `proptest` for fuzzing

### Test Patterns
```rust
// Use tempfile for file operations
let dir = tempdir().unwrap();
let file_path = dir.path().join("test.rs");
fs::write(&file_path, "code here").unwrap();

// Use SearchOptions for search functions
let options = SearchOptions {
    extensions: Some(vec!["rs".to_string()]),
    ..Default::default()
};
```

### Code Quality Standards
- **100% test pass rate** required (173 unit + 36 integration tests)
- **Zero clippy warnings** (run `cargo clippy -- -D warnings`)
- Average module size: ~200 LOC (maintainability target)

## Common Development Patterns

### Adding New Search Features
1. Extend `SearchOptions` struct in `src/types/mod.rs`
2. Implement logic in appropriate `search/` sub-module
3. Add unit tests in module's `tests` section
4. Update CLI args in `src/cli/mod.rs`

### Adding New Analysis Types
1. Create new module following `deadcode/` pattern:
   ```
   your_feature/
   ├── mod.rs       # Public API and orchestration
   ├── types.rs     # Data structures
   ├── helpers.rs   # Utility functions
   ├── detectors.rs # Detection logic (if applicable)
   ```
2. Export public API in `src/lib.rs`
3. Add command handler in `src/commands/`
4. Add integration test in `tests/`

### Working with Graph Analysis
Graph modules follow a consistent pattern:
1. `analyze_file()` - Single file analysis
2. `build_*_from_source()` - Build graph from source code
3. Export graph data structures for visualization

### MCP Integration
To add new MCP tools:
1. Define parameter struct with `JsonSchema` derive
2. Implement tool function in `src/mcp/tools.rs`
3. Register with `tool_router` in `src/mcp/server.rs`

## Performance Considerations

- **Parallel Processing**: Always use `par_iter()` for file operations
- **Caching**: Use `DashMap` for thread-safe caching
- **Streaming**: Use streaming file reading for large files (`memopt/`)
- **Regex**: Compile regex patterns outside loops
- **String Operations**: Use `Cow<str>` to avoid unnecessary cloning

## Important Notes

- **Rust Edition**: 2024 (requires Rust 1.70+)
- **Optional Features**: `mcp` feature flag required for AI integration
- **Thread Safety**: All shared state uses `Arc` and `DashMap`
- **Error Handling**: Use `anyhow` for errors, `thiserror` for custom error types
- **Target**: Native binary only (no WASM support)

## Module Dependencies (High-Level)

Core modules are independent:
- `search/` can work standalone
- `deadcode/`, `duplicates/`, `complexity/` depend on `search/` and `parser/`
- `analysis/` depends on most other modules
- `mcp/` depends on `search/` and `analysis/`

When adding features, prefer:
- Pure functions in `pure/` sub-modules
- Parallel processing in core logic
- Thread-safe data structures (DashMap) for shared state
- Options struct pattern for complex parameters
