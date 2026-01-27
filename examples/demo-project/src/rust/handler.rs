/// Handler module - part of circular dependency
use super::processor::{process_data, transform_and_process};

/// Handle incoming requests
pub fn handle_request(request: &str) -> Result<String, String> {
    if request.is_empty() {
        return Err("Empty request".to_string());
    }

    // Circular call back to processor
    let processed = process_data(request)?;

    // Another circular call
    validate_and_forward(&processed)
}

/// Forward to validation (circular)
fn validate_and_forward(data: &str) -> Result<String, String> {
    if data.len() > 100 {
        return Err("Data too large".to_string());
    }

    // Call back into processor (circular dependency)
    transform_and_process(data)
}

/// Export validation for circular calls
pub fn validate_request(request: &str) -> bool {
    !request.contains("invalid") && request.len() > 0
}
