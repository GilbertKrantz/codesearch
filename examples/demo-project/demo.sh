#!/bin/bash

# CodeSearch Demo Script
# This script demonstrates all major CodeSearch capabilities

set -e

echo "========================================"
echo "CodeSearch Comprehensive Demo"
echo "========================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print section headers
print_section() {
    echo ""
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SRC_DIR="$SCRIPT_DIR/src"

# Check if codesearch is built
if ! command -v codesearch &> /dev/null; then
    echo "codesearch not found in PATH. Trying to use from project root..."
    PROJECT_ROOT="$SCRIPT_DIR/../.."
    if [ -f "$PROJECT_ROOT/target/release/codesearch" ]; then
        CODESEARCH="$PROJECT_ROOT/target/release/codesearch"
    elif [ -f "$PROJECT_ROOT/target/debug/codesearch" ]; then
        CODESEARCH="$PROJECT_ROOT/target/debug/codesearch"
    else
        echo "Error: codesearch binary not found. Please build the project first:"
        echo "  cd $PROJECT_ROOT && cargo build --release"
        exit 1
    fi
else
    CODESEARCH="codesearch"
fi

echo -e "${GREEN}Using CodeSearch from: $CODESEARCH${NC}"
echo ""

# ============================================================================
# 1. BASIC SEARCH
# ============================================================================
print_section "1. Basic Search Capabilities"

echo -e "${YELLOW}1a. Simple search for 'function' keyword:${NC}"
$CODESEARCH "function" "$SRC_DIR" | head -20

echo ""
echo -e "${YELLOW}1b. Case-sensitive search for 'Calculator':${NC}"
$CODESEARCH --case-sensitive "Calculator" "$SRC_DIR"

echo ""
echo -e "${YELLOW}1c. Regex search for function definitions:${NC}"
$CODESEARCH --regex "fn\s+\w+" "$SRC_DIR/rust"

echo ""
echo -e "${YELLOW}1d. Fuzzy search (with typo) - 'calcualtor':${NC}"
$CODESEARCH --fuzzy "calcualtor" "$SRC_DIR"

# ============================================================================
# 2. CODE ANALYSIS
# ============================================================================
print_section "2. Code Analysis Features"

echo -e "${YELLOW}2a. Dead code detection:${NC}"
$CODESEARCH deadcode "$SRC_DIR"

echo ""
echo -e "${YELLOW}2b. Complexity analysis:${NC}"
$CODESEARCH complexity "$SRC_DIR"

echo ""
echo -e "${YELLOW}2c. Code duplicates detection:${NC}"
$CODESEARCH duplicates "$SRC_DIR" --threshold 0.7

echo ""
echo -e "${YELLOW}2d. Circular dependency detection:${NC}"
$CODESEARCH circular "$SRC_DIR/rust" || true

echo ""
echo -e "${YELLOW}2e. Codebase metrics and analysis:${NC}"
$CODESEARCH analyze "$SRC_DIR"

# ============================================================================
# 3. GRAPH ANALYSIS
# ============================================================================
print_section "3. Graph Analysis"

echo -e "${YELLOW}3a. Control Flow Graph (CFG) for calculator.rs:${NC}"
$CODESEARCH cfg "$SRC_DIR/rust/calculator.rs" --format text | head -30

echo ""
echo -e "${YELLOW}3b. Call Graph for Rust modules:${NC}"
$CODESEARCH callgraph "$SRC_DIR/rust" --format text | head -30

echo ""
echo -e "${YELLOW}3c. Abstract Syntax Tree (AST):${NC}"
$CODESEARCH ast "$SRC_DIR/rust/calculator.rs" --format text | head -20

# ============================================================================
# 4. EXPORT FORMATS
# ============================================================================
print_section "4. Export Formats"

echo -e "${YELLOW}4a. Export dead code to CSV:${NC}"
OUTPUT_CSV="$SCRIPT_DIR/deadcode_report.csv"
$CODESEARCH deadcode "$SRC_DIR" --format csv --output "$OUTPUT_CSV"
echo "CSV report saved to: $OUTPUT_CSV"
head -5 "$OUTPUT_CSV"

echo ""
echo -e "${YELLOW}4b. Export analysis to Markdown:${NC}"
OUTPUT_MD="$SCRIPT_DIR/analysis_report.md"
$CODESEARCH analyze "$SRC_DIR" --format markdown --output "$OUTPUT_MD"
echo "Markdown report saved to: $OUTPUT_MD"
head -20 "$OUTPUT_MD"

# ============================================================================
# 5. ADVANCED SEARCH
# ============================================================================
print_section "5. Advanced Search Features"

echo -e "${YELLOW}5a. Search with ranking:${NC}"
$CODESEARCH --rank "process" "$SRC_DIR"

echo ""
echo -e "${YELLOW}5b. Search specific file types (Rust only):${NC}"
$CODESEARCH --extensions rs "fn\|pub" "$SRC_DIR"

echo ""
echo -e "${YELLOW}5c. Search for TODOs and FIXMEs:${NC}"
$CODESEARCH --regex "TODO|FIXME" "$SRC_DIR"

# ============================================================================
# 6. LANGUAGE-SPECIFIC SEARCH
# ============================================================================
print_section "6. Multi-Language Support"

echo -e "${YELLOW}6a. Search in Python files:${NC}"
$CODESEARCH --extensions py "def\|class" "$SRC_DIR"

echo ""
echo -e "${YELLOW}6b. Search in TypeScript files:${NC}"
$CODESEARCH --extensions ts "interface\|class\|function" "$SRC_DIR"

echo ""
echo -e "${YELLOW}6c. Search in configuration files:${NC}"
$CODESEARCH --extensions yaml,toml "enabled\|true" "$SRC_DIR"

# ============================================================================
# 7. CODE QUALITY METRICS
# ============================================================================
print_section "7. Code Quality Metrics"

echo -e "${YELLOW}7a. Design metrics (coupling, cohesion):${NC}"
$CODESEARCH design-metrics "$SRC_DIR" || true

echo ""
echo -e "${YELLOW}7b. Comprehensive code metrics:${NC}"
$CODESEARCH metrics "$SRC_DIR" || true

# ============================================================================
# 8. EXAMPLE FINDINGS
# ============================================================================
print_section "8. Key Findings in Demo Project"

echo -e "${YELLOW}Dead Code Found:${NC}"
echo "  - calculator.rs: unused_function(), deprecated_method()"
echo "  - data_processor.py: _legacy_method(), legacy_process()"
echo "  - UserService.ts: legacyAuthenticate(), deprecatedCacheKey()"
echo ""

echo -e "${YELLOW}High Complexity Functions:${NC}"
echo "  - calculator.rs: calculate() (cyclomatic complexity > 10)"
echo "  - data_processor.py: process_items() (deep nesting: 5 levels)"
echo "  - UserService.ts: addUser() (deep nesting: 7 levels)"
echo ""

echo -e "${YELLOW}Code Duplicates:${NC}"
echo "  - process_addition / process_subtraction patterns (multiple files)"
echo "  - Similar data processing logic across Rust, Python, TypeScript"
echo ""

echo -e "${YELLOW}Circular Dependencies:${NC}"
echo "  - Rust: circular.rs module system"
echo "  - Python: circular_a.py <-> circular_b.py"
echo "  - TypeScript: DataFlow.ts <-> Pipeline.ts"
echo ""

echo -e "${YELLOW}Code Smells:${NC}"
echo "  - TODO markers: 8 instances"
echo "  - FIXME markers: 3 instances"
echo "  - Magic numbers: Multiple instances"
echo "  - Empty methods: clear_cache(), clear_history()"
echo "  - Commented-out code: 3 instances"
echo ""

# ============================================================================
# 9. PERFORMANCE DEMO
# ============================================================================
print_section "9. Performance Features"

echo -e "${YELLOW}9a. Search with benchmarking:${NC}"
$CODESEARCH --benchmark "function" "$SRC_DIR"

echo ""
echo -e "${YELLOW}9b. File listing:${NC}"
$CODESEARCH files "$SRC_DIR"

# ============================================================================
# SUMMARY
# ============================================================================
print_section "Demo Summary"

echo -e "${GREEN}CodeSearch capabilities demonstrated:${NC}"
echo "✓ Basic search (simple, regex, fuzzy, case-sensitive)"
echo "✓ Code analysis (dead code, complexity, duplicates)"
echo "✓ Graph analysis (CFG, Call Graph, AST, DFG)"
echo "✓ Export formats (CSV, Markdown, JSON)"
echo "✓ Advanced search (ranking, filtering)"
echo "✓ Multi-language support (48+ languages)"
echo "✓ Code quality metrics"
echo "✓ Circular dependency detection"
echo ""

echo -e "${GREEN}Demo project statistics:${NC}"
echo "- Languages: Rust, Python, TypeScript, YAML"
echo "- Files: 12 source files"
echo "- Dead code items: 15+"
echo "- High complexity functions: 6+"
echo "- Code duplicates: 3+ clone pairs"
echo "- Circular dependencies: 3 module pairs"
echo ""

echo -e "${BLUE}Demo complete!${NC}"
echo ""
echo "Generated reports:"
echo "  - $OUTPUT_CSV"
echo "  - $OUTPUT_MD"
echo ""
echo "Try exploring the demo project yourself:"
echo "  cd $SCRIPT_DIR"
echo "  $CODESEARCH interactive"
