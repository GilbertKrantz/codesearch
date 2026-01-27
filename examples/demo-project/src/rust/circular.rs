/// Circular dependency example
/// Demonstrates: circular function calls, call graph analysis

pub mod processor;
pub mod handler;

use handler::handle_request;
use processor::process_data;

/// Main entry point that triggers circular calls
pub fn main_workflow(input: &str) -> String {
    println!("Starting workflow with input: {}", input);
    process_data(input)
}

/// Public API that chains circular calls
pub fn execute_pipeline(data: &str) -> Result<String, String> {
    handle_request(data)
}

/// Helper function that's part of the circular call chain
fn validate_data(data: &str) -> bool {
    !data.is_empty()
}

/// Another function in the call chain
fn transform_data(data: &str) -> String {
    data.to_uppercase()
}
