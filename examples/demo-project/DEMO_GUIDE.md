# CodeSearch Demo Guide

A comprehensive guide demonstrating all CodeSearch capabilities using the example project.

## Table of Contents

1. [Setup](#setup)
2. [Quick Start](#quick-start)
3. [Search Capabilities](#search-capabilities)
4. [Code Analysis](#code-analysis)
5. [Graph Analysis](#graph-analysis)
6. [Export Formats](#export-formats)
7. [Advanced Features](#advanced-features)
8. [Use Cases](#use-cases)

## Setup

### Prerequisites

1. **Build CodeSearch** (if not already installed):
   ```bash
   cd /path/to/codesearch
   cargo build --release
   ```

2. **Navigate to demo project**:
   ```bash
   cd examples/demo-project
   ```

### Quick Demo Run

Run the automated demo script:
```bash
./demo.sh
```

This will execute all CodeSearch features on the example codebase.

## Quick Start

### Basic Search

Search for a simple pattern:
```bash
codesearch "function" ./src
```

### Interactive Mode

Launch interactive search:
```bash
codesearch interactive
```

## Search Capabilities

### 1. Simple Search

Search for text patterns across files:
```bash
# Search for "Calculator"
codesearch "Calculator" ./src

# Search in specific directory
codesearch "process" ./src/rust
```

### 2. Case-Sensitive Search

Enable case-sensitive matching:
```bash
codesearch --case-sensitive "Calculator" ./src
```

### 3. Regex Search

Use regular expressions for complex patterns:
```bash
# Find function definitions in Rust
codesearch --regex "fn\s+\w+" ./src/rust

# Find class definitions in Python
codesearch --regex "class\s+\w+" ./src/python

# Find interface definitions in TypeScript
codesearch --regex "interface\s+\w+" ./src/typescript
```

### 4. Fuzzy Search

Find patterns even with typos:
```bash
# Handles typos - will find "Calculator"
codesearch --fuzzy "calcualtor" ./src

# Set custom threshold (0.0-1.0)
codesearch --fuzzy --threshold 0.6 "processer" ./src
```

### 5. File Extension Filtering

Search specific file types:
```bash
# Only Rust files
codesearch --extensions rs "pub fn" ./src

# Multiple extensions
codesearch --extensions rs,py,ts "function" ./src

# Config files
codesearch --extensions yaml,toml "enabled" ./src
```

### 6. Search with Ranking

Sort results by relevance:
```bash
codesearch --rank "algorithm" ./src
```

## Code Analysis

### Dead Code Detection

Find unused code:
```bash
# Basic dead code detection
codesearch deadcode ./src

# With output format
codesearch deadcode ./src --format csv --output deadcode.csv

# Specific language
codesearch deadcode ./src/rust
```

**What it finds:**
- Unused functions and methods
- Unused variables
- Unused imports
- Unreachable code
- Empty functions
- Commented-out code

**Example findings in demo project:**
```
calculator.rs:98 - unused_function()
calculator.rs:103 - deprecated_method()
data_processor.py:62 - _legacy_method()
UserService.ts:98 - legacyAuthenticate()
```

### Complexity Analysis

Analyze code complexity:
```bash
# Cyclomatic complexity
codesearch complexity ./src

# Specific file
codesearch complexity ./src/rust/calculator.rs
```

**What it measures:**
- Cyclomatic complexity (branching complexity)
- Cognitive complexity (human comprehension difficulty)
- Deep nesting levels
- Function length

**Example findings:**
```
calculator.rs:32 - calculate() - Complexity: 15 (High)
data_processor.py:43 - process_items() - Complexity: 12 (High)
UserService.ts:35 - addUser() - Complexity: 14 (High)
```

### Duplicate Detection

Find code clones:
```bash
# Detect duplicates with default threshold (0.7)
codesearch duplicates ./src

# Custom threshold
codesearch duplicates ./src --threshold 0.8

# Only Type-1 clones (exact duplicates)
codesearch duplicates ./src --type 1
```

**Clone Types:**
- **Type-1**: Exact copies (whitespace included)
- **Type-2**: Structurally identical (variable names may differ)
- **Type-3**: Modified copies (insertions/deletions)

**Example findings:**
```
Clone Pair (Type-2, Similarity: 85%):
  - calculator.rs:124 - process_addition()
  - calculator.rs:135 - process_subtraction()
```

### Codebase Analysis

Get overall metrics:
```bash
codesearch analyze ./src
```

**Output includes:**
- Total files analyzed
- Total lines of code
- Language distribution
- File size statistics
- Code quality overview

## Graph Analysis

### 1. Control Flow Graph (CFG)

Analyze execution paths:
```bash
# Text format
codesearch cfg ./src/rust/calculator.rs

# DOT format for visualization
codesearch cfg ./src/rust/calculator.rs --format dot --output cfg.dot

# View with Graphviz
dot -Tpng cfg.dot -o cfg.png
```

**What it shows:**
- Basic blocks
- Branching conditions
- Loop structures
- Unreachable code
- Back edges (loops)

### 2. Call Graph

Analyze function relationships:
```bash
# Text format
codesearch callgraph ./src/rust

# DOT format
codesearch callgraph ./src/rust --format dot --output callgraph.dot
```

**What it shows:**
- Function call relationships
- Circular dependencies
- Entry points
- Dead functions (never called)
- Call depth

### 3. Abstract Syntax Tree (AST)

Analyze code structure:
```bash
codesearch ast ./src/rust/calculator.rs
```

**What it shows:**
- Function/class declarations
- Control structures
- Variable declarations
- Expression trees

### 4. Data Flow Graph (DFG)

Analyze variable dependencies:
```bash
codesearch dfg ./src/rust/calculator.rs
```

**What it shows:**
- Variable definitions
- Variable uses
- Data dependencies
- Unused variables
- Redundant computations

### 5. Program Dependency Graph (PDG)

Combined control and data dependencies:
```bash
codesearch pdg ./src/rust/calculator.rs
```

### 6. All Graphs

Generate all graph types:
```bash
codesearch graph-all ./src/rust/calculator.rs
```

### Circular Dependency Detection

Find circular function calls:
```bash
codesearch circular ./src/rust
```

**Example findings:**
```
Circular call chain detected:
  process_a() -> process_b() -> process_a()
```

## Export Formats

### CSV Export

```bash
# Dead code to CSV
codesearch deadcode ./src --format csv --output results.csv

# Custom CSV
codesearch "TODO" ./src --format csv --output todos.csv
```

**CSV columns:**
- File
- Line
- Type
- Description
- Severity

### Markdown Export

```bash
# Analysis to Markdown
codesearch analyze ./src --format markdown --output report.md

# Complexity report
codesearch complexity ./src --format markdown --output complexity.md
```

### JSON Export

```bash
# Search results as JSON
codesearch "function" ./src --format json --output results.json
```

### DOT Export (Graphs)

```bash
# Control flow graph
codesearch cfg ./src/rust/calculator.rs --format dot --output cfg.dot

# Visualize with Graphviz
dot -Tpng cfg.dot -o cfg.png
```

## Advanced Features

### Semantic Search

Context-aware search:
```bash
codesearch --semantic "error handling" ./src
```

### Search with Caching

Enable caching for faster repeated searches:
```bash
codesearch --cache "function" ./src
```

### Benchmark Mode

Measure search performance:
```bash
codesearch --benchmark "test" ./src
```

### Comparison with grep

Compare CodeSearch vs traditional grep:
```bash
codesearch --vs-grep "pattern" ./src
```

## Use Cases

### Use Case 1: Code Review

Before reviewing code:
```bash
# Find potential issues
codesearch deadcode ./src --format markdown --output review.md
codesearch complexity ./src >> review.md
codesearch duplicates ./src >> review.md
```

### Use Case 2: Refactoring

Before refactoring:
```bash
# Understand dependencies
codesearch callgraph ./src --format dot --output deps.dot

# Find similar code to consolidate
codesearch duplicates ./src --threshold 0.8
```

### Use Case 3: Code Quality Audit

Comprehensive audit:
```bash
#!/bin/bash
codesearch deadcode ./src --output audit_deadcode.csv
codesearch complexity ./src --output audit_complexity.csv
codesearch duplicates ./src --output audit_duplicates.csv
codesearch circular ./src --output audit_circular.txt
```

### Use Case 4: Learning a Codebase

Quick exploration:
```bash
# List all files
codesearch files ./src

# Find main entry points
codesearch --regex "(main|def main|fn main|int main)" ./src

# Understand call structure
codesearch callgraph ./src
```

### Use Case 5: Finding Tech Debt

Search for debt markers:
```bash
# Find TODOs and FIXMEs
codesearch --regex "TODO|FIXME|HACK|XXX" ./src

# Find commented code
codesearch --regex "^[[:space:]]*//.*code|^[[:space:]]*#.*code" ./src

# Find temporary/debug code
codesearch --regex "console\.log|println!|debugger" ./src
```

## Demo Project Details

### File Structure

```
demo-project/
├── src/
│   ├── rust/
│   │   ├── calculator.rs        # High complexity, dead code
│   │   ├── circular.rs          # Circular dependency module
│   │   ├── handler.rs           # Part of circular dep
│   │   └── processor.rs         # Part of circular dep
│   ├── python/
│   │   ├── data_processor.py    # Complexity, dead code
│   │   ├── circular_a.py        # Circular imports
│   │   └── circular_b.py        # Circular imports
│   ├── typescript/
│   │   ├── UserService.ts       # Complexity, dead code
│   │   ├── DataFlow.ts          # Circular dependency
│   │   └── Pipeline.ts          # Circular dependency
│   └── config/
│       └── app.yaml             # Configuration example
├── README.md                    # This file
└── demo.sh                      # Automated demo script
```

### Intentional Issues

The demo project contains intentional code quality issues:

1. **Dead Code**: 15+ instances
   - Unused functions
   - Unused variables
   - Unused imports
   - Unreachable code

2. **High Complexity**: 6+ functions
   - Deep nesting (5-7 levels)
   - Multiple conditions
   - Complex logic

3. **Code Duplicates**: 3+ clone pairs
   - Type-2 clones (structural duplicates)
   - Similar patterns across languages

4. **Circular Dependencies**: 3 pairs
   - Rust: module system circular calls
   - Python: circular imports
   - TypeScript: circular imports

5. **Code Smells**: 20+ instances
   - TODO markers (8)
   - FIXME markers (3)
   - Magic numbers (multiple)
   - Empty methods (2)
   - Commented-out code (3)

## Tips and Tricks

### 1. Combine Multiple Searches

Use shell pipes to combine searches:
```bash
codesearch "function" ./src | grep "calculator"
```

### 2. Save Common Patterns

Create a script with common searches:
```bash
#!/bin/bash
# my-code-review.sh
codesearch deadcode ./src
codesearch complexity ./src
codesearch --regex "TODO|FIXME" ./src
```

### 3. Use with Build Tools

Integrate with CI/CD:
```bash
# Fail build on high complexity
complexity=$(codesearch complexity ./src --format json)
if [ $(echo "$complexity" | jq '.max_complexity') -gt 15 ]; then
    echo "High complexity detected!"
    exit 1
fi
```

### 4. Generate Documentation

Automatically generate documentation:
```bash
# Export all analyses
codesearch analyze ./src --format markdown --output docs/analysis.md
codesearch callgraph ./src --format dot --output docs/callgraph.dot
```

## Additional Resources

- **Main Documentation**: See the project's main README.md
- **Language Support**: `codesearch languages` lists all 48+ supported languages
- **Help**: `codesearch --help` for command-line options
- **Command Help**: `codesearch <command> --help` for command-specific help

## Conclusion

This demo project showcases CodeSearch's comprehensive capabilities for:

- **Fast, intelligent code search** (regex, fuzzy, semantic)
- **Automated code analysis** (dead code, complexity, duplicates)
- **Graph-based analysis** (CFG, DFG, call graphs, AST)
- **Multiple export formats** (CSV, Markdown, JSON, DOT)
- **Multi-language support** (48+ programming languages)

Use this demo as a reference for implementing CodeSearch in your own projects!
