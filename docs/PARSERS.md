# Native Parser Documentation

CodeSearch includes high-performance native parsers for major programming languages. These parsers provide accurate AST analysis without external dependencies.

## Overview

Native parsers offer:
- **Zero-allocation tokenization** for maximum performance
- **Accurate AST extraction** for functions, classes, imports, and variables
- **Language-specific features** (async/await, generics, annotations)
- **Integration with graph analysis** (CFG, DFG, call graphs)

## Supported Languages

### Rust Parser

**Extensions:** `.rs`

**Features:**
- Function declarations with visibility modifiers (`pub`, `pub(crate)`)
- Async functions
- Structs with fields
- Enums and variants
- Impl blocks with methods
- Use statements (imports)
- Variable declarations (`let`, `const`, `static`)
- Generic types and lifetimes

**Example:**
```rust
pub async fn fetch_data() -> Result<Data, Error> {
    // Detected as: public async function with return type
}

pub struct Point {
    x: f64,  // Detected as field
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        // Detected as method in impl block
    }
}
```

### Python Parser

**Extensions:** `.py`, `.pyw`, `.pyi`

**Features:**
- Function definitions with decorators
- Async functions
- Class definitions with inheritance
- Import statements (`import`, `from...import`)
- Variable assignments
- Type annotations
- Docstrings

**Example:**
```python
async def fetch_data() -> dict:
    # Detected as: async function with return type
    pass

class Point:
    def __init__(self, x: float, y: float):
        # Detected as: class with constructor
        self.x = x
        self.y = y
```

### JavaScript/TypeScript Parser

**Extensions:** `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs`

**Features:**
- Function declarations and expressions
- Arrow functions
- Async/await
- Classes with methods and fields
- ES6 imports/exports
- TypeScript interfaces and types
- JSX/TSX syntax

**Example:**
```javascript
export async function fetchData() {
    // Detected as: exported async function
}

class Point {
    constructor(x, y) {
        this.x = x;  // Detected as field
        this.y = y;
    }
    
    distance() {
        // Detected as method
    }
}

import { useState } from 'react';  // Detected as import
```

### Go Parser

**Extensions:** `.go`

**Features:**
- Function declarations
- Methods with receivers
- Struct definitions
- Interface definitions
- Import statements
- Variable declarations (`var`, `const`)
- Package declarations
- Exported vs unexported (capitalization)

**Example:**
```go
type Point struct {
    X float64  // Detected as exported field
    Y float64
}

func (p *Point) Distance() float64 {
    // Detected as: method with pointer receiver
}

func NewPoint(x, y float64) *Point {
    // Detected as: exported function (capitalized)
}
```

### Java Parser

**Extensions:** `.java`

**Features:**
- Class declarations
- Interface declarations
- Enum definitions
- Method declarations with modifiers
- Field declarations
- Import statements
- Annotations
- Visibility modifiers (public, private, protected)

**Example:**
```java
public class Point {
    private double x;  // Detected as private field
    private double y;
    
    public Point(double x, double y) {
        // Detected as: public constructor
    }
    
    public double distance() {
        // Detected as: public method with return type
    }
}
```

## Usage

### Programmatic API

```rust
use codesearch::parser::{CodeParser, RustParser, PythonParser};

// Parse Rust code
let parser = RustParser;
let analysis = parser.parse_content(rust_code)?;
println!("Functions: {}", analysis.functions.len());
println!("Classes: {}", analysis.classes.len());

// Get parser by extension
use codesearch::parser::get_parser_for_extension;
if let Some(parser) = get_parser_for_extension("py") {
    let functions = parser.extract_functions(python_code);
}
```

### CLI Usage

Native parsers are automatically used when analyzing supported file types:

```bash
# AST analysis uses native parser for .rs files
codesearch ast src/main.rs

# Call graph uses native parsers for accurate function detection
codesearch callgraph ./src --extensions rs,py,js,go,java

# Complexity analysis benefits from accurate parsing
codesearch complexity ./src
```

## Performance

Native parsers are optimized for speed:

| Parser | Typical Speed | Features |
|--------|--------------|----------|
| Rust | ~50-100 µs per file | Zero-allocation tokenizer |
| Python | ~60-120 µs per file | Indentation-aware |
| JavaScript | ~55-110 µs per file | ES6+ support |
| Go | ~45-95 µs per file | Interface detection |
| Java | ~65-130 µs per file | Annotation support |

*Benchmarks on typical source files (200-500 lines)*

Run benchmarks:
```bash
cargo bench --bench parser_benchmarks
```

## Architecture

### Tokenizer

All parsers share a common tokenizer (`src/parser/tokenizer.rs`) that:
- Performs zero-allocation scanning
- Handles comments (line and block)
- Recognizes keywords, identifiers, literals, operators
- Tracks line and column positions

### Parser Trait

Each parser implements the `CodeParser` trait:

```rust
pub trait CodeParser: Send + Sync {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError>;
    fn extract_functions(&self, content: &str) -> Vec<FunctionInfo>;
    fn extract_classes(&self, content: &str) -> Vec<ClassInfo>;
    fn extract_imports(&self, content: &str) -> Vec<ImportInfo>;
    fn extract_variables(&self, content: &str) -> Vec<VariableInfo>;
    fn language_name(&self) -> &'static str;
    fn extensions(&self) -> &[&'static str];
}
```

### AST Types

Parsed information is returned in structured types:

```rust
pub struct FunctionInfo {
    pub name: String,
    pub line: usize,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_public: bool,
}

pub struct ClassInfo {
    pub name: String,
    pub line: usize,
    pub methods: Vec<String>,
    pub fields: Vec<String>,
    pub is_public: bool,
}
```

## Graph Integration

Native parsers enhance graph analysis:

### Call Graph
- Accurate function detection across languages
- Method resolution for OOP languages
- Receiver tracking for Go methods

### Control Flow Graph (CFG)
- Language-specific branch detection
- Loop identification (for, while, loop)
- Return statement tracking

### Data Flow Graph (DFG)
- Variable definition tracking
- Assignment detection
- Scope analysis

## Testing

Each parser includes comprehensive tests:

```bash
# Run all parser tests
cargo test --lib parser

# Run specific parser tests
cargo test --lib parser::rust::tests
cargo test --lib parser::python::tests
cargo test --lib parser::javascript::tests
cargo test --lib parser::go::tests
cargo test --lib parser::java::tests
```

## Adding New Parsers

To add a new language parser:

1. Create `src/parser/language.rs`
2. Implement `CodeParser` trait
3. Add language keywords array
4. Implement parsing methods
5. Add tests
6. Update `src/parser/mod.rs` exports
7. Update `get_parser_for_extension()`

Example skeleton:

```rust
use crate::ast::{AstAnalysis, ClassInfo, FunctionInfo, ImportInfo, VariableInfo};
use crate::parser::error::ParseError;
use crate::parser::token::{Token, TokenKind};
use crate::parser::traits::CodeParser;
use crate::parser::tokenizer::Tokenizer;

const LANG_KEYWORDS: &[&str] = &["keyword1", "keyword2"];

pub struct LanguageParser;

impl CodeParser for LanguageParser {
    fn parse_content(&self, content: &str) -> Result<AstAnalysis, ParseError> {
        let mut tokenizer = Tokenizer::new(content, LANG_KEYWORDS);
        let tokens = tokenizer.tokenize();
        // Parse tokens...
    }
    
    fn language_name(&self) -> &'static str {
        "Language"
    }
    
    fn extensions(&self) -> &[&'static str] {
        &["ext"]
    }
}
```

## Limitations

Current parsers focus on top-level declarations:
- Function/method signatures (not full bodies)
- Class/struct definitions (not all members)
- Import statements
- Variable declarations

For deeper analysis, consider:
- Using tree-sitter for full AST parsing
- Language-specific tools (rustc, pylint, etc.)
- IDE language servers

## Future Enhancements

Planned improvements:
- [ ] C/C++ native parser
- [ ] Ruby native parser
- [ ] PHP native parser
- [ ] Method body parsing for CFG
- [ ] Full expression parsing for DFG
- [ ] Incremental parsing for large files
- [ ] Error recovery for partial parses

## References

- [Parser Module](../src/parser/)
- [Tokenizer Implementation](../src/parser/tokenizer.rs)
- [Parser Benchmarks](../benches/parser_benchmarks.rs)
- [AST Types](../src/ast.rs)
