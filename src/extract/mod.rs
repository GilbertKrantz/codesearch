//! Regex-based code extraction for analysis modules
//!
//! Provides lightweight extraction of functions, classes, and references from source code.

mod regex_extractor;

pub use regex_extractor::{
    extract_classes, extract_function_calls, extract_functions, extract_identifier_from_match,
    extract_identifier_references, is_keyword_or_builtin,
};
