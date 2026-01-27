/// Calculator module with various complexity patterns
/// Demonstrates: high complexity, dead code, code smells

// TODO: Add support for floating point numbers
// FIXME: This implementation has precision issues with large numbers

use std::collections::HashMap;

/// Unused import - dead code
use std::vec::Vec;

/// Simple calculator for basic arithmetic operations
#[derive(Debug)]
pub struct Calculator {
    // Magic number - should be a named constant
    precision: u32,
    history: Vec<String>,
}

impl Calculator {
    /// Create a new calculator
    pub fn new() -> Self {
        Self {
            precision: 10,
            history: Vec::new(),
        }
    }

    /// Add two numbers - high complexity function
    /// This function demonstrates high cyclomatic complexity
    pub fn calculate(&self, op: &str, a: i64, b: i64) -> Result<i64, String> {
        match op {
            "add" => {
                // Deep nesting level 1
                if a > 0 {
                    // Deep nesting level 2
                    if b > 0 {
                        // Deep nesting level 3
                        if a + b > 1000 {
                            // Deep nesting level 4
                            Ok(a + b)
                        } else {
                            Ok(a + b)
                        }
                    } else {
                        Ok(a + b)
                    }
                } else {
                    Ok(a + b)
                }
            }
            "subtract" => Ok(a - b),
            "multiply" => {
                // Multiple conditions
                if a == 0 || b == 0 {
                    Ok(0)
                } else if a == 1 {
                    Ok(b)
                } else if b == 1 {
                    Ok(a)
                } else if a < 0 && b < 0 {
                    Ok(a * b)
                } else if a < 0 {
                    Ok(a * b)
                } else if b < 0 {
                    Ok(a * b)
                } else {
                    Ok(a * b)
                }
            }
            "divide" => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    if a % b == 0 {
                        Ok(a / b)
                    } else {
                        Ok(a / b)
                    }
                }
            }
            "power" => {
                if b == 0 {
                    Ok(1)
                } else if b < 0 {
                    Err(String::from("Negative exponents not supported"))
                } else {
                    let mut result = 1;
                    for _ in 0..b {
                        result *= a;
                    }
                    Ok(result)
                }
            }
            "modulo" => {
                if b == 0 {
                    Err(String::from("Division by zero"))
                } else {
                    Ok(a % b)
                }
            }
            _ => Err(String::from("Unknown operation")),
        }
    }

    /// Unused function - dead code
    fn unused_function(&self) {
        println!("This is never called");
    }

    /// Another unused function
    fn deprecated_method(&self) -> i32 {
        42
    }

    /// Empty function - code smell
    pub fn clear_history(&mut self) {
        // TODO: Implement history clearing
    }

    /// Function with redundant computation
    pub fn calculate_statistics(&self, numbers: &[i64]) -> (i64, i64, f64) {
        let sum: i64 = numbers.iter().sum();
        let count = numbers.len() as i64;

        // Redundant computation
        let average = if count > 0 {
            sum as f64 / count as f64
        } else {
            0.0
        };

        let max = numbers.iter().max().unwrap_or(&0);
        let min = numbers.iter().min().unwrap_or(&0);

        (sum, *max, average)
    }
}

/// Duplicate code pattern 1 - Type 2 clone (syntactically identical but different variables)
fn process_addition(a: i64, b: i64) -> i64 {
    let result = a + b;
    println!("Processing addition: {} + {} = {}", a, b, result);
    if result > 1000 {
        println!("Large result detected");
    }
    result
}

/// Duplicate code pattern 2 - Type 2 clone
fn process_subtraction(a: i64, b: i64) -> i64 {
    let result = a - b;
    println!("Processing subtraction: {} - {} = {}", a, b, result);
    if result < 0 {
        println!("Negative result detected");
    }
    result
}

/// Unused variable - dead code
pub fn unused_variables() {
    let active = true;
    let count = 42;
    let message = "Hello";

    let _result = count * 2;
}

/// Unreachable code
pub fn unreachable_example(x: i32) -> i32 {
    if x > 10 {
        return 1;
    } else {
        return 2;
    }

    // Unreachable
    println!("This will never be printed");
    999
}

/// Commented out code - code smell
pub fn commented_code(x: i32) -> i32 {
    /*
    let old_version = x * 2;
    if old_version > 100 {
        return old_version;
    }
    */

    // New implementation
    x * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let calc = Calculator::new();
        assert_eq!(calc.calculate("add", 2, 3), Ok(5));
    }

    #[test]
    fn test_division_by_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.calculate("divide", 10, 0), Err(String::from("Division by zero")));
    }
}
