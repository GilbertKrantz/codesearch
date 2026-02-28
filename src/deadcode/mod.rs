//! Dead Code Detection Module
//!
//! Identifies potentially unused code: functions, classes, imports, and commented-out code.
//!
//! Sub-modules:
//! - `types`: Data structures for dead code items
//! - `helpers`: Utility functions for detection
//! - `detectors`: Individual detection functions for different code patterns
//! - `detector_impl`: Core find_dead_code logic

mod detector_impl;
mod detectors;
mod helpers;
mod types;

pub use detector_impl::find_dead_code;
pub use types::DeadCodeItem;

use colored::*;
use std::collections::HashMap;
use std::path::Path;

/// Detect potentially dead/unused code in the codebase
pub fn detect_dead_code(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Dead Code Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let dead_code_items = find_dead_code(path, extensions, exclude)?;

    if dead_code_items.is_empty() {
        println!("{}", "No files found to analyze.".dimmed());
        return Ok(());
    }

    print_dead_code_results(&dead_code_items);

    Ok(())
}

fn print_dead_code_results(items: &[DeadCodeItem]) {
    if items.is_empty() {
        println!("{}", "No obvious dead code detected!".green().bold());
    } else {
        println!(
            "{}",
            format!("Found {} potential dead code items:", items.len())
                .yellow()
                .bold()
        );
        println!();

        let mut current_file = String::new();
        for item in items {
            if item.file != current_file {
                current_file = item.file.clone();
                println!("{}", format!("[{}]", current_file).cyan());
            }
            println!(
                "   {} L{}: {} '{}' - {}",
                match item.item_type.as_str() {
                    "function" => "[fn]",
                    "class/struct" => "[cls]",
                    "variable" => "[var]",
                    "import" => "[imp]",
                    "unreachable" => "[!]",
                    "empty" => "[∅]",
                    "todo" => "[?]",
                    "parameter" => "[prm]",
                    _ => "[-]",
                },
                format!("{:4}", item.line_number).yellow(),
                item.item_type.blue(),
                item.name.green(),
                item.reason.dimmed()
            );
        }

        println!();
        println!("{}", "Summary:".cyan().bold());

        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for item in items {
            *type_counts.entry(item.item_type.clone()).or_insert(0) += 1;
        }

        for (item_type, count) in &type_counts {
            println!("   {} {}: {}", "•".dimmed(), item_type, count);
        }
    }
}
