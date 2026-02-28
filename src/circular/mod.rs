//! Circular Call Detection Module
//!
//! Detects circular function calls (cycles in the call graph).

mod detector;
mod types;

pub use detector::{deduplicate_cycles, find_circular_calls, find_cycles_dfs, format_cycle};
pub use types::CircularCall;

use colored::*;
use std::path::Path;

/// Detect circular function calls and print results
pub fn detect_circular_calls(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<(), Box<dyn std::error::Error>> {
    let cycles = find_circular_calls(path, extensions, exclude)?;

    println!("{}", "Circular Call Detection".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    if cycles.is_empty() {
        println!("{}", "No circular calls detected!".green().bold());
    } else {
        println!(
            "{}",
            format!("Found {} circular call chain(s):", cycles.len())
                .yellow()
                .bold()
        );
        println!();

        for (i, cycle) in cycles.iter().enumerate() {
            println!("  {}. {}", i + 1, format_cycle(&cycle.chain).red());
            for file in &cycle.files {
                println!("     - {}", file.dimmed());
            }
            println!();
        }

        println!("{}", "─".repeat(50).dimmed());
        println!(
            "{} {} circular call chain(s) found",
            "-".dimmed(),
            cycles.len().to_string().yellow().bold()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_cycle() {
        let chain = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(format_cycle(&chain), "a -> b -> c -> a");
    }

    #[test]
    fn test_deduplicate_cycles() {
        let cycles = vec![
            CircularCall {
                chain: vec!["a".to_string(), "b".to_string()],
                files: vec!["f1.rs".to_string()],
            },
            CircularCall {
                chain: vec!["b".to_string(), "a".to_string()],
                files: vec!["f1.rs".to_string()],
            },
        ];
        let unique = deduplicate_cycles(cycles);
        assert_eq!(unique.len(), 1);
    }
}
