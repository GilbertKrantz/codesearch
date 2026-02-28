//! Command Handlers Module
//!
//! This module contains handlers for all CLI commands, extracted from main.rs
//! for better maintainability and testability.

pub mod analysis;
pub mod graph;
pub mod search;

pub use analysis::{handle_analyze_command, handle_complexity_command, handle_deadcode_command};
pub use graph::{handle_cfg_command, handle_dfg_command, handle_graph_command};
pub use search::handle_search_command;
