/// Processor module - part of circular dependency
use super::handler::{handle_request, validate_request};

/// Process data - circular call to handler
pub fn process_data(data: &str) -> Result<String, String> {
    println!("Processing data: {}", data);

    // Circular call back to handler
    if validate_request(data) {
        Ok(format!("Processed: {}", data))
    } else {
        Err("Invalid data".to_string())
    }
}

/// Transform and process with circular dependency
pub fn transform_and_process(data: &str) -> Result<String, String> {
    let transformed = data.to_uppercase();

    // Circular call back to handler
    handle_request(&transformed)
}

/// Additional processing function
pub fn enhance_data(data: &str) -> String {
    format!("Enhanced: {}", data)
}

/// Dead code - unused function
pub fn legacy_process(data: &str) -> String {
    data.to_string()
}
