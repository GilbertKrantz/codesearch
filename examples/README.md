# Code Search Examples

This directory contains example code files in various programming languages to demonstrate and test the code search tool capabilities.

## Example Files

### Rust (`rust_example.rs`)
- Demonstrates struct definitions, implementations, and methods
- Shows error handling patterns
- Contains test modules
- Keywords: `struct`, `impl`, `fn`, `pub`, `Result`, `Error`

### Python (`python_example.py`)
- Object-oriented programming with classes and dataclasses
- Type hints and enums
- Logging and JSON handling
- Keywords: `class`, `def`, `import`, `from`, `@dataclass`, `Enum`

### JavaScript (`javascript_example.js`)
- ES6+ features including classes, arrow functions, and async/await
- Event handling and DOM manipulation
- Map data structure and array methods
- Keywords: `class`, `function`, `async`, `await`, `const`, `let`

### Dead Code Demos (`deadcode_demo.rs`, `deadcode_demo.py`)
- Intentionally contain unused imports, functions, classes, and constants
- Used to demonstrate the `codesearch deadcode` command
- Compare used vs unused code patterns

### TypeScript (`typescript_example.ts`)
- Interfaces, generics, and type annotations
- Custom error classes and API response types
- Async/await patterns with proper error handling
- Keywords: `interface`, `type`, `class`, `async`, `await`, `enum`

### Go (`go_example.go`)
- Structs, methods, and interfaces
- Error handling and validation
- JSON marshaling and regular expressions
- Keywords: `struct`, `func`, `interface`, `map`, `error`

### Java (`java_example.java`)
- Classes, inheritance, and polymorphism
- Exception handling and collections
- Stream API and lambda expressions
- Keywords: `class`, `public`, `private`, `static`, `interface`, `extends`

## Testing the Code Search Tool

Use these examples to test various search patterns:

### Basic Text Search
```bash
# Search for function definitions
codesearch "function" examples --extensions js,ts

# Search for class definitions
codesearch "class" examples --extensions py,js,ts,java

# Search for struct definitions
codesearch "struct" examples --extensions rs,go
```

### Regex Patterns
```bash
# Find all function definitions
codesearch search "fn\\s+\\w+" examples --extensions rs

# Find all public methods
codesearch search "pub\\s+fn" examples --extensions rs

# Find all async functions
codesearch search "async\\s+function" examples --extensions js,ts

# Find all class methods
codesearch search "def\\s+\\w+" examples --extensions py
```

### Case-Insensitive Search
```bash
# Search for error handling patterns
codesearch search "error" examples --ignore-case --extensions rs,py,js,ts,go,java

# Search for validation patterns
codesearch search "validate" examples --ignore-case
```

### File Filtering
```bash
# List all example files
codesearch files examples

# List only Rust files
codesearch files examples --extensions rs

# List only Python and JavaScript files
codesearch files examples --extensions py,js
```

### JSON Output
```bash
# Get structured output for analysis
codesearch search "class" examples --extensions py,js,ts,java --format json

# Search for imports and exports
codesearch search "import|export" examples --extensions js,ts --format json
```

### Complex Patterns
```bash
# Find all error types
codesearch search "Error|Exception" examples --extensions rs,py,js,ts,go,java

# Find all API endpoints or routes
codesearch search "app\\.|router\\.|@app\\.|@router\\." examples --extensions py,js,ts

# Find all test functions
codesearch search "test_|@test|func Test" examples --extensions py,js,ts,go

# Find all configuration or constants
codesearch search "const|CONST|config|Config" examples --ignore-case
```

## Search Tips

1. **Use regex for complex patterns**: The tool supports full regex syntax
2. **Combine file extensions**: Use comma-separated extensions to search multiple languages
3. **Use line numbers**: Add `--line-numbers` to see context
4. **Case-insensitive search**: Use `--ignore-case` for broader matches
5. **Limit results**: Use `--max-results` to control output size
6. **Exclude directories**: Use `--exclude` to skip build directories

### Dead Code Detection Demo Files

Two dedicated demo files (`deadcode_demo.rs` and `deadcode_demo.py`) contain intentional dead code:

- **`deadcode_demo.rs`**: Unused imports, structs, enums, and functions
- **`deadcode_demo.py`**: Unused imports, constants, classes, and functions

```bash
# Detect dead code in demo files
codesearch deadcode examples

# Detect dead code in specific language
codesearch deadcode examples --extensions rs
codesearch deadcode examples --extensions py
```

**Types of dead code detected:**

1. **Unused Imports**: Modules imported but never referenced
2. **Unused Functions**: Functions defined but never called
3. **Unused Classes/Structs**: Types defined but never instantiated
4. **Single-use Functions**: Functions called only once (inlining candidates)

**Example output:**
```
🔍 Dead Code Detection
──────────────────────────────

⚠️  Found 5 potential dead code items:

📄 examples/deadcode_demo.py
   📥 L   5: import 'os' - Imported but never used
   📥 L   6: import 'sys' - Imported but never used
📄 examples/deadcode_demo.rs
   📥 L   4: import 'HashMap' - Imported but never used
   📥 L   6: import 'Write' - Imported but never used
```

## Example Use Cases

- **Code Review**: Find all TODO comments or error handling patterns
- **Refactoring**: Locate all instances of a function or class
- **Documentation**: Find all public APIs and interfaces
- **Testing**: Locate all test functions and test data
- **Security**: Search for potential security issues or hardcoded secrets
- **Architecture**: Understand code structure and dependencies
- **Code Cleanup**: Identify and remove dead code with `codesearch deadcode`
