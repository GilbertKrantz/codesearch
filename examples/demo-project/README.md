# CodeSearch Demo Project

A comprehensive example project demonstrating CodeSearch's capabilities.

## Project Structure

```
demo-project/
├── src/
│   ├── rust/           # Rust code examples
│   ├── python/         # Python code examples
│   ├── typescript/     # TypeScript code examples
│   └── config/         # Configuration files
└── demo.sh             # Demo script
```

## Demo Scenarios

This codebase contains intentional examples of:

### Code Quality Issues
- **Dead Code**: Unused functions, variables, imports
- **High Complexity**: Functions with high cyclomatic complexity
- **Code Duplication**: Similar code blocks across files
- **Code Smells**: TODOs, FIXMEs, magic numbers, deep nesting

### Graph Analysis Examples
- **Control Flow**: Complex branching, loops, unreachable code
- **Data Flow**: Variable dependencies, data transformations
- **Call Graph**: Circular calls, function relationships
- **Dependencies**: Module imports and relationships

## Running the Demo

```bash
# Navigate to demo directory
cd examples/demo-project

# Run the automated demo script
./demo.sh

# Or run individual commands:
```

## Example CodeSearch Commands

### Basic Search
```bash
# Simple search
codesearch "function" ./src

# Regex search
codesearch --regex "fn\s+\w+" ./src/rust

# Fuzzy search (handles typos)
codesearch --fuzzy "calcualtor" ./src

# Case-sensitive search
codesearch --case-sensitive "Calculator" ./src
```

### Code Analysis
```bash
# Dead code detection
codesearch deadcode ./src

# Complexity analysis
codesearch complexity ./src

# Duplicate detection
codesearch duplicates ./src

# Codebase metrics
codesearch analyze ./src
```

### Graph Analysis
```bash
# Control Flow Graph
codesearch cfg ./src/rust/calculator.rs --format dot

# Call Graph
codesearch callgraph ./src/rust/ --format dot

# All graphs
codesearch graph-all ./src/rust/calculator.rs
```

### Export Options
```bash
# Export to CSV
codesearch "TODO" ./src --format csv --output results.csv

# Export to Markdown
codesearch analyze ./src --format markdown --output analysis.md

# Export to JSON
codesearch search "function" ./src --format json
```

### Advanced Features
```bash
# Interactive mode
codesearch interactive

# Semantic search (context-aware)
codesearch --semantic "error handling" ./src

# With ranking
codesearch --rank "algorithm" ./src

# Performance benchmark
codesearch --benchmark "test" ./src
```
