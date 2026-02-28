//! Data types for circular call detection

use serde::Serialize;

/// A circular call chain
#[derive(Debug, Clone, Serialize)]
pub struct CircularCall {
    pub chain: Vec<String>,
    pub files: Vec<String>,
}
