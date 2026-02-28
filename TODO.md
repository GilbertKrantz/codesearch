# TODO

## ✅ Completed

- [x] Implement basic code search functionality
- [x] Add fuzzy search support
- [x] Add interactive mode
- [x] Add codebase analysis
- [x] Add refactoring suggestions
- [x] Implement MCP server support (rmcp 0.10 with 9 tools: search, list, analyze, complexity, duplicates, deadcode, circular, find_symbol, get_health)
- [x] Add comprehensive unit tests (80+ tests)
- [x] Add integration tests (26 tests)
- [x] Simplify CLI usage with defaults
- [x] Add semantic search enhancement
- [x] Add caching system for performance
- [x] Update README with comprehensive documentation
- [x] Create architecture documentation
- [x] Add progress indicators for long-running searches
- [x] Add export functionality (CSV, Markdown)
- [x] Add keyboard shortcuts in interactive mode
- [x] Add code complexity metrics (cyclomatic & cognitive complexity)
- [x] Add code duplication detection
- [x] Add dead code detection with enhanced capabilities:
  - Unused variables and constants detection
  - Unreachable code detection (after return/break/continue)
  - Empty function detection (supports Python, Rust, JS, etc.)
  - TODO/FIXME/HACK/XXX/BUG marker detection
  - Commented-out code detection
  - Unused import detection
- [x] Add comprehensive multi-language support (48 languages)
- [x] Modularize codebase into smaller maintainable modules (19 modules)
- [x] Refactor deadcode.rs into modular structure (4 sub-modules for better maintainability)
- [x] Extract CLI definitions from main.rs to cli.rs module (reduced main.rs from 1050 to 624 lines)
- [x] Modularize codemetrics.rs into 5 submodules (complexity, size, maintainability, helpers, mod)
- [x] Modularize designmetrics.rs into 5 submodules (types, analysis, extractors, reporting, mod)
- [x] Modularize language.rs into 4 submodules (types, definitions, utilities, mod)
- [x] Modularize search.rs into 5 submodules (core, fuzzy, semantic, utilities, mod)
- [x] Remove unsubstantiated performance claims from documentation
- [x] Ensure all key capabilities are exposed to MCP (7 tools total)
- [x] Verify code maintainability and testability standards
- [x] Implement all 6 graph analysis types:
  - Abstract Syntax Tree (AST)
  - Control Flow Graph (CFG)
  - Data Flow Graph (DFG)
  - Call Graph
  - Dependency Graph (enhanced)
  - Program Dependency Graph (PDG)
- [x] Add unified graph analysis interface
- [x] Add CLI commands for all graph types
- [x] Add DOT format export for visualization
- [x] Add 22 unit tests for graph modules
- [x] Implement design metrics module:
  - Afferent Coupling (Ca)
  - Efferent Coupling (Ce)
  - Instability (I)
  - Abstractness (A)
  - Distance from Main Sequence (D)
  - Package Cohesion (LCOM)
- [x] Add CLI command for design metrics analysis
- [x] Add 6 unit tests for design metrics
- [x] Implement comprehensive code metrics module:
  - Cyclomatic Complexity
  - Halstead Metrics (11 sub-metrics)
  - Essential Complexity
  - NPath Complexity
  - Lines of Code (LOC, SLOC, LLOC)
  - Code Density & Comment Ratio
  - Maintainability Index (MI)
  - Code Churn
  - Depth of Inheritance Tree (DIT)
  - Coupling Between Objects (CBO)
  - Lack of Cohesion in Methods (LCOM)
- [x] Add CLI command for comprehensive metrics
- [x] Add 4 unit tests for code metrics
- [x] Code quality improvements (Jan 2026):
  - Fixed 100+ clippy warnings across the codebase
  - Removed useless comparisons in tests (>= 0 for unsigned types)
  - Converted to inline format args for better readability
  - Fixed never-looping for loops to use if-let patterns
  - Replaced manual min/max with clamp() function
  - Removed unused imports (VecDeque, Revwalk, graph types)
  - Moved regex compilation outside loops for performance
  - Improved code consistency and maintainability

## 🔄 In Progress

- None currently

## 📋 Planned

### Capability Redesign (from docs/CAPABILITY_REDESIGN.md)

**Differentiator:** "The local code lens for terminal users and AI agents"

- [x] **Phase 1: Consolidation** ✅
  - [x] Add `codesearch health` (deadcode + duplicates + complexity)
  - [x] Add `codesearch graph <cfg|dfg|dep|ast|pdg>` as unified entry point
  - [x] Merge `metrics` into `analyze` (--metrics flag)
  - [x] Deprecate `remote`, `design-metrics`, `graph-all`, `metrics`

- [x] **Phase 2: Structural Find** ✅
  - [x] Implement `codesearch find <symbol>` (definition, references, callers)
  - [x] Add `--type definition|callers|references`
  - [x] JSON output for piping

- [x] **Phase 3: Health Scoring** ✅
  - [x] Health score formula (0-100)
  - [x] `--fail-under` for CI gates
  - [x] Structured JSON output

- [x] **Phase 4: MCP Expansion** ✅
  - [x] Add `find_symbol` MCP tool
  - [x] Add `get_health` MCP tool
  - [x] Update README: MCP first-class

### Maintainability Improvements (High Priority)
- [x] **Extract trait abstractions for core components** ✅ (Jan 2026)
  - ✅ Created `SearchEngine` trait for different search strategies
  - ✅ Created `Analyzer` trait for different analysis types
  - ✅ Created `GraphBuilder` trait for graph construction
  - ✅ Implemented `DefaultSearchEngine` wrapping existing search_code
  - ✅ Added comprehensive documentation with examples
  - ✅ Included mock implementations for testing
  - Benefits: Better testability, easier to mock, clearer contracts

- [x] **Reduce function parameter counts** ✅ (Jan 2026)
  - ~~`search_code()` has 13 parameters (limit: 7)~~
  - ✅ Introduced `SearchOptions` struct to bundle related parameters
  - ✅ Applied builder pattern with `with_*` methods
  - ✅ Reduced `search_code()` from 13 parameters to 3 parameters
  - ✅ Updated all 15+ call sites across the codebase
  - ✅ All 173 tests pass

- [x] **Split large modules into focused sub-modules** ✅ (Jan 2026)
  - ✅ Extracted command handlers from `main.rs` into `commands/` module
  - ✅ Created 3 sub-modules: `search.rs`, `analysis.rs`, `graph.rs`
  - ✅ Reduced main.rs complexity by moving 200+ LOC to handlers
  - ✅ Added comprehensive documentation to all handlers
  - ✅ Included tests for each command handler
  - Pattern: Follows `deadcode/`, `codemetrics/`, `search/` module structure

- [x] **Improve error handling consistency** ✅ (Jan 2026)
  - ✅ Defined custom error types using `thiserror`
  - ✅ Created 4 error enums: `SearchError`, `AnalysisError`, `GraphError`, `RemoteError`
  - ✅ Added 8+ specific error variants per type
  - ✅ Implemented error source chains for debugging
  - ✅ Added automatic conversions from common error types
  - ✅ Created comprehensive example in `examples/error_handling.rs`
  - ✅ Documented error handling patterns with 5 examples
  - Note: Full migration to custom errors is gradual (backward compatible)

- [x] **Add documentation for public APIs** ✅ (Jan 2026)
  - ✅ Added comprehensive rustdoc to all command handlers
  - ✅ Documented FileSystem trait with usage examples
  - ✅ Added module-level documentation to commands/
  - ✅ Included examples in all public function docs
  - ✅ Ready for `cargo doc` generation
  - Note: Ongoing - will continue adding docs to remaining modules

### Test-Friendliness Improvements (High Priority)
- [x] **Introduce dependency injection** ✅ (Jan 2026)
  - ✅ Created `FileSystem` trait with 10 operations
  - ✅ Implemented `RealFileSystem` for production use
  - ✅ Implemented `MockFileSystem` for testing (in-memory)
  - ✅ All traits are `Send + Sync` for thread safety
  - ✅ Added comprehensive documentation and examples
  - ✅ Included 5 tests demonstrating mock usage

- [x] **Extract testable pure functions** ✅ (Jan 2026)
  - ✅ Created `search/pure.rs` module with pure functions
  - ✅ Extracted `calculate_relevance_score_pure` (no I/O)
  - ✅ Added `relevance_category`, `fuzzy_match_quality`, `should_include_line`
  - ✅ All functions are independently testable
  - ✅ Included 8 unit tests for pure functions

- [x] **Add property-based testing** ✅ (Jan 2026)
  - ✅ Added `proptest` dependency to Cargo.toml
  - ✅ Created `tests/proptest_search.rs` with 7 property tests
  - ✅ Tests verify: no panics, query in results, max results respected
  - ✅ Tests cover: fuzzy threshold, extension filters, empty queries
  - ✅ Generates random inputs to find edge cases

- [x] **Improve test isolation** ✅ (Jan 2026)
  - ✅ Created `tests/fixtures/mod.rs` with reusable fixtures
  - ✅ Implemented `TestWorkspace` for temporary test directories
  - ✅ Added sample code snippets (Rust, Python, JavaScript)
  - ✅ All tests use `tempfile` for isolation
  - ✅ No shared state between tests
  - ✅ Included 4 tests for fixture functionality

- [x] **Add integration test coverage** ✅ (Jan 2026)
  - ✅ Created `tests/integration_e2e.rs` with 15 end-to-end tests
  - ✅ Tests cover: search→export, multi-extension, fuzzy matching
  - ✅ Tests analyze→search workflow, complexity analysis
  - ✅ Tests deadcode detection, ranking, exclusions
  - ✅ Tests case sensitivity, nested directories, empty dirs
  - ✅ All tests use fixtures for isolation

- [x] **Add test coverage reporting** ✅ (Jan 2026)
  - ✅ Added `tarpaulin` to dev-dependencies
  - ✅ Created `tarpaulin.toml` configuration
  - ✅ Set minimum coverage threshold at 70%
  - ✅ Created GitHub Actions workflow for CI/CD
  - ✅ Configured HTML, LCOV, and JSON output formats
  - ✅ Excludes test files from coverage metrics

### Performance Improvements (Medium Priority)
- [x] Add incremental indexing for large codebases
- [x] Implement file watching for real-time updates
- [x] Optimize memory usage for very large files

- [x] **Optimize hot paths** ✅ (Jan 2026)
  - ✅ Added `criterion` for benchmarking
  - ✅ Created `benches/search_benchmark.rs` with 6 benchmarks
  - ✅ Benchmarks cover: small/medium searches, relevance scoring
  - ✅ Benchmarks test fuzzy matching, pure functions
  - ✅ Ready for profiling with `cargo bench`
  - Note: Use `cargo flamegraph` for detailed profiling

- [ ] **Improve parallel processing**
  - Tune rayon thread pool size based on workload
  - Use work-stealing for better load balancing
  - Consider async I/O for network operations (remote search)

- [x] **Enhance caching strategy** ✅ (Jan 2026)
  - ✅ Implemented `LruCacheWrapper` in `cache_lru.rs`
  - ✅ Thread-safe LRU cache with automatic eviction
  - ✅ Prevents unbounded memory growth
  - ✅ Configurable capacity
  - ✅ Included 9 tests for LRU functionality
  - ✅ Ready to replace simple cache in search module
  - Pre-compile common patterns at startup
  - Use `regex::RegexSet` for multiple pattern matching
  - Consider using `aho-corasick` for literal string matching

- [ ] **Reduce memory allocations**
  - Use string interning for repeated strings (file paths)
  - Reuse buffers in hot loops
  - Use `Cow<str>` to avoid unnecessary cloning

### Features
- [x] Add AST-based code analysis (beyond regex)
- [x] Add dependency graph analysis
- [x] Add support for git history search
- [x] Add support for searching in remote repositories

### User Experience
- [ ] Add search result preview pane

### Testing
- [x] Add MCP server integration tests (23 tests)
- [ ] Add performance benchmarks
- [ ] Add fuzz testing for edge cases
- [ ] Add more complex integration test scenarios
- [ ] Add test coverage reporting

### Documentation
- [ ] Add API documentation (rustdoc)
- [ ] Add more usage examples
- [ ] Add architecture decision records (ADRs)

## 🐛 Known Issues

- None currently

## 💡 Ideas for Future

### Architecture Evolution
- [ ] **Workspace crate structure** (for very large projects)
  - Split into `codesearch-core`, `codesearch-cli`, `codesearch-mcp`
  - Share common types via `codesearch-types` crate
  - Benefits: Faster compilation, better modularity

- [ ] **Plugin system**
  - Allow external search strategies via dynamic loading
  - Custom analyzers for domain-specific languages
  - Third-party graph visualizers

### Advanced Features
- [ ] Machine learning-based code pattern recognition
- [ ] Collaborative search patterns sharing
- [ ] Code search as a service (web API)
- [ ] Integration with code review tools
- [ ] Support for searching in binary files (with limits)
- [ ] Add support for searching in database schemas
- [ ] Add support for searching in configuration files

### Quality Metrics
- [ ] Track technical debt over time
- [ ] Code health dashboard
- [ ] Automated refactoring suggestions with diffs

