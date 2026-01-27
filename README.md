# CodeSearch

**Fast, intelligent code search and analysis for 48+ programming languages.**

Find what you need in seconds: functions, classes, duplicates, dead code, complexity issues.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

---

## Why CodeSearch?

### **Stop Wasting Time Searching Code**

**Problem:** You're working in a large codebase and need to:
- Find where authentication logic is implemented
- Identify all usages of a deprecated function before refactoring
- Track down technical debt (TODOs, FIXMEs) scattered across files
- Understand complex function relationships and dependencies
- Find duplicated code that violates DRY principles
- Spot overly complex functions that need refactoring

**Traditional tools fall short:**
- `grep` is slow and doesn't understand code structure
- IDE search is limited to single projects/languages
- Manual code review is time-consuming and error-prone

**CodeSearch solves these problems:**

```bash
# Find authentication logic instantly
codesearch "authenticate" ./src --rank
# Result: All authentication code, ranked by relevance

# Track technical debt before sprint planning
codesearch deadcode ./src --format csv --output debt.csv
# Result: 15 unused functions, 23 TODOs, 8 unreachable blocks

# Find duplicates before they become maintenance nightmares
codesearch duplicates ./src
# Result: 12 code clones (80%+ similarity)
```

### **What Makes CodeSearch Different?**

| Feature | Benefit | Example |
|---------|---------|---------|
| **Language-Aware** | Understands functions, classes, imports in 48+ languages | Find `fn main` in Rust, `def main` in Python |
| **Lightning Fast** | Parallel processing with Rust, typical searches in 3-50ms | Search 1000 files in < 50ms |
| **Intelligent** | Fuzzy matching handles typos, semantic search understands context | `codesearch "authetication"` finds "authentication" |
| **Code Quality** | Detects dead code, duplicates, complexity issues automatically | `codesearch complexity` flags functions needing refactoring |
| **Graph Analysis** | 6 types of graphs for deep code understanding | Call graphs show function relationships |
| **Developer-Friendly** | Interactive mode, multiple export formats, MCP for AI agents | `codesearch interactive` for REPL-style search |

### **Real-World Impact**

- **Save Hours per Week**: Replace manual code hunting with instant searches
- **Ship Better Code**: Catch dead code and complexity issues before review
- **Understand Faster**: Visualize code relationships with graph analysis
- **Reduce Technical Debt**: Track and eliminate code quality issues systematically

---

## Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/yingkitw/codesearch.git
cd codesearch
cargo build --release

# The binary will be at: ./target/release/codesearch
# Optional: Add to PATH
export PATH="$PATH:$PWD/target/release"
```

### Basic Usage

```bash
# Simple search - find anything in your codebase
codesearch "function" ./src

# Search with file type filter
codesearch "class" ./src --extensions rs,py

# Fuzzy search (handles typos!)
codesearch "calcualtor" ./src --fuzzy

# Interactive mode
codesearch interactive
```

---

## Usage Examples

### 1. **Everyday Search Tasks**

#### Find Function Definitions
```bash
# Find all functions named "process"
codesearch "fn process" ./src --extensions rs

# Find class definitions
codesearch "class User" ./src --extensions py

# Find async functions
codesearch --regex "async\s+fn\s+\w+" ./src --extensions rs
```

#### Track Technical Debt
```bash
# Find all TODOs and FIXMEs
codesearch "TODO\|FIXME" ./src

# Export to CSV for tracking
codesearch "TODO" ./src --format csv --output todos.csv
```

#### Refactor Safely
```bash
# Find all usages before refactoring
codesearch "old_function_name" ./src

# Case-sensitive search for exact matches
codesearch "MyStruct" ./src --case-sensitive
```

### 2. **Code Quality Analysis**

#### Detect Dead Code
```bash
# Find unused code
codesearch deadcode ./src

# Output:
# ⚠️ Found 12 potential dead code items:
#    [var] L 10: variable 'unused_var'
#    [∅] L 42: empty_helper()
#    [?] L 58: TODO marker
#    [!] L 72: unreachable code

# Export for code review
codesearch deadcode ./src --format markdown --output review.md
```

#### Analyze Complexity
```bash
# Find complex functions that need refactoring
codesearch complexity ./src

# Output:
# 📊 Files by Complexity:
#   src/auth.rs: Cyclomatic 45, Cognitive 38 (HIGH)
#   src/parser.rs: Cyclomatic 28, Cognitive 22 (MEDIUM)
```

#### Find Code Duplicates
```bash
# Identify copy-pasted code
codesearch duplicates ./src

# Output:
# 🔍 Found 8 duplicate code blocks:
#   auth.rs:120-145 vs user.rs:89-114 (85% similar)
```

### 3. **Understanding Codebases**

#### Codebase Overview
```bash
# Get high-level metrics
codesearch analyze ./src

# Output:
# Overview
#   Total files: 156
#   Total lines: 45,230
#   Languages: Rust (60%), Python (25%), TypeScript (15%)
#   Functions: 892
#   Classes: 124
```

#### Explore Function Relationships
```bash
# Generate call graph
codesearch callgraph ./src --format text

# Output:
# Call Graph Analysis:
#   Functions: 28
#   Function calls: 156
#   Recursive: authenticate()
#   Dead (never called): legacy_auth()
```

#### Control Flow Analysis
```bash
# Understand execution paths
codesearch cfg ./src/auth.rs --format text

# Shows: Basic blocks, branches, loops, unreachable code
```

### 4. **Advanced Workflows**

#### Interactive Mode
```bash
codesearch interactive

# Commands in interactive mode:
# authenticate       - Search for "authenticate"
# /f                 - Toggle fuzzy matching
# /i                 - Toggle case sensitivity
# analyze            - Show codebase metrics
# complexity         - Show complexity analysis
# deadcode           - Find dead code
# help               - Show all commands
```

#### Search with Ranking
```bash
# Get results ranked by relevance
codesearch "auth" ./src --rank

# Best match first:
#   src/auth/mod.rs:10 - pub fn authenticate() { ... }
#   src/user.rs:45     - fn check_auth() { ... }
```

#### Export Results
```bash
# CSV for spreadsheets
codesearch "TODO" ./src --format csv --output todos.csv

# Markdown for documentation
codesearch analyze ./src --format markdown --output analysis.md

# JSON for automation
codesearch "function" ./src --format json | jq '.matches'
```

### 5. **Special Features**

#### Search Git History
```bash
# Search through commit history
codesearch git-history "TODO" ./src
```

#### Search Remote Repositories
```bash
# Search GitHub/GitLab without cloning
codesearch remote --github "pattern" owner/repo
```

#### Build Index for Large Codebases
```bash
# Incremental indexing for faster searches
codesearch index ./src

# Watch for changes and auto-update
codesearch watch ./src
```

---

## Common Commands Reference

### Search Commands
```bash
codesearch "<query>" [path] [options]

# Options:
--extensions, -e     # Filter by file extensions (e.g., rs,py,js)
--case-sensitive     # Case-sensitive matching
--fuzzy              # Fuzzy matching (handles typos)
--regex              # Use regular expressions
--rank               # Rank results by relevance
--format             # Output format (text, csv, markdown, json)
--output, -o         # Output file path
```

### Analysis Commands
```bash
codesearch deadcode [path]          # Find unused code
codesearch complexity [path]        # Analyze complexity
codesearch duplicates [path]        # Find duplicates
codesearch analyze [path]           # Codebase metrics
codesearch circular [path]          # Circular dependencies
codesearch design-metrics [path]    # Coupling & cohesion
codesearch metrics [path]           # All metrics
```

### Graph Commands
```bash
codesearch ast [file]               # Abstract Syntax Tree
codesearch cfg [file]               # Control Flow Graph
codesearch dfg [file]               # Data Flow Graph
codesearch callgraph [path]         # Call Graph
codesearch depgraph [path]          # Dependency Graph
codesearch pdg [file]               # Program Dependency Graph
codesearch graph-all [file]         # All graph types
```

### Utility Commands
```bash
codesearch files [path]             # List searchable files
codesearch languages                # List supported languages
codesearch interactive              # Interactive mode
codesearch index [path]             # Build index
codesearch watch [path]             # Watch for changes
codesearch git-history <query>      # Search git history
codesearch remote --github <query>  # Search GitHub
```

---

## Real-World Examples

### Example 1: Pre-Code Review Checklist

```bash
#!/bin/bash
# review.sh - Automated code review checklist

echo "=== Code Review Report ==="
echo ""

echo "1. Dead Code Issues:"
codesearch deadcode ./src --format markdown

echo ""
echo "2. Complexity Issues:"
codesearch complexity ./src --threshold 15

echo ""
echo "3. Duplicate Code:"
codesearch duplicates ./src

echo ""
echo "4. Technical Debt:"
codesearch "TODO\|FIXME\|HACK" ./src
```

### Example 2: Learning a New Codebase

```bash
# Step 1: Understand the structure
codesearch analyze ./src

# Step 2: Find entry points
codesearch --regex "(main|Main|app\.start)" ./src

# Step 3: Explore key modules
codesearch files ./src --extensions rs | head -20

# Step 4: Understand function relationships
codesearch callgraph ./src --format text | head -50

# Step 5: Find complex code to review
codesearch complexity ./src --sort | head -10
```

### Example 3: Refactoring Workflow

```bash
# Before refactoring, find all usages
codesearch "OldAuthService" ./src --output old_usage.txt

# Check for complexity issues
codesearch complexity ./src/auth --format markdown > complexity_report.md

# Find similar code that could be consolidated
codesearch duplicates ./src/auth --output duplicates.txt

# After refactoring, verify no old code remains
codesearch "OldAuthService" ./src
# Should return: "No matches found"
```

### Example 4: Continuous Quality Monitoring

```bash
# Add to CI/CD pipeline

# Fail if high complexity functions detected
complexity=$(codesearch complexity ./src --format json)
max_cc=$(echo "$complexity" | jq '.max_complexity')
if [ "$max_cc" -gt 20 ]; then
    echo "❌ High complexity detected: $max_cc"
    exit 1
fi

# Fail if new dead code introduced
deadcode_count=$(codesearch deadcode ./src --format json | jq '.total')
if [ "$deadcode_count" -gt 10 ]; then
    echo "❌ Too much dead code: $deadcode_count items"
    exit 1
fi

echo "✅ Code quality checks passed"
```

---

## Demo Project

A comprehensive example project demonstrating all CodeSearch capabilities is available in the [examples/demo-project/](examples/demo-project/) directory.

**Run the demo:**
```bash
cd examples/demo-project
./demo.sh
```

**Demo includes:**
- Multi-language codebase (Rust, Python, TypeScript)
- Intentional code quality issues for detection
- All analysis types demonstrated
- Real-world usage examples

---

## Architecture & Quality

### Code Quality Standards
- ✅ **100% test pass rate** (173 unit + 36 integration tests)
- ✅ **Zero clippy warnings** (clean code)
- ✅ **Modular architecture** (40+ focused modules)
- ✅ **Thread-safe** parallel processing with rayon
- ✅ **Comprehensive error handling**

### Performance
- **Fast**: 3-50ms for typical searches (< 1000 files)
- **Parallel**: Auto-scales to available CPU cores
- **Smart caching**: 70-90% cache hit rate for repeated searches
- **Memory efficient**: Streaming file reading, < 100MB for 10K files

### Supported Languages

**Native Parsers (High Performance):**
- **Rust** - Full AST parsing with zero-allocation tokenizer
- **Python** - Complete syntax support including async/await
- **JavaScript/TypeScript** - ES6+, JSX, TSX support
- **Go** - Structs, interfaces, methods with receivers
- **Java** - Classes, interfaces, enums, annotations

**48+ Additional Languages** via regex patterns including: C/C++, Ruby, PHP, Swift, Kotlin, C#, Haskell, Elixir, Erlang, Scala, Lua, Perl, Shell, SQL, YAML, TOML, JSON, and more.

See `codesearch languages` for the complete list.

---

## Additional Resources

- [**Demo Project**](examples/demo-project/) - Hands-on examples
- [**DEMO_GUIDE.md**](examples/demo-project/DEMO_GUIDE.md) - Comprehensive usage guide
- [**ARCHITECTURE.md**](ARCHITECTURE.md) - Technical details and design
- [**CLAUDE.md**](CLAUDE.md) - Contributor guide

---

## License

Apache-2.0 License

---

**Built with Rust** • Fast • Precise • 48+ Languages
