//! Unified codebase health scan
//!
//! Combines dead code, duplicates, and complexity into a single "debt scan"
//! with an optional 0-100 health score for CI gates.

use crate::complexity::calculate_file_complexity;
use crate::deadcode::find_dead_code;
use crate::duplicates::find_duplicates;
use crate::parser::read_file_content;
use crate::search::list_files;
use serde::Serialize;
use std::path::Path;

/// Result of a health scan
#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    pub score: u8,
    pub dead_code_count: usize,
    pub duplicate_count: usize,
    pub complex_files_count: usize,
    pub total_files: usize,
    pub details: HealthDetails,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthDetails {
    pub dead_code_penalty: u8,
    pub duplicate_penalty: u8,
    pub complexity_penalty: u8,
}

const COMPLEXITY_THRESHOLD: u32 = 15;
const MAX_PENALTY_PER_CATEGORY: u8 = 25;

/// Run a health scan and return structured results
pub fn scan_health(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<HealthReport, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;

    if files.is_empty() {
        return Ok(HealthReport {
            score: 100,
            dead_code_count: 0,
            duplicate_count: 0,
            complex_files_count: 0,
            total_files: 0,
            details: HealthDetails {
                dead_code_penalty: 0,
                duplicate_penalty: 0,
                complexity_penalty: 0,
            },
        });
    }

    let dead_items = find_dead_code(path, extensions, exclude)?;
    let dead_code_count = dead_items.len();

    let duplicates = find_duplicates(path, extensions, exclude, 3, 0.85)?;
    let duplicate_count = duplicates.len();

    let mut complex_files_count = 0;
    for file in &files {
        let content = read_file_content(&file.path);
        let metrics = calculate_file_complexity(&file.path, &content);
        if metrics.cyclomatic_complexity >= COMPLEXITY_THRESHOLD {
            complex_files_count += 1;
        }
    }

    let dead_penalty = (dead_code_count * 2).min(MAX_PENALTY_PER_CATEGORY as usize) as u8;
    let dup_penalty = (duplicate_count * 5).min(MAX_PENALTY_PER_CATEGORY as usize) as u8;
    let compl_penalty = (complex_files_count * 5).min(MAX_PENALTY_PER_CATEGORY as usize) as u8;

    let total_penalty = dead_penalty
        .saturating_add(dup_penalty)
        .saturating_add(compl_penalty);
    let score = 100u8.saturating_sub(total_penalty);

    Ok(HealthReport {
        score,
        dead_code_count,
        duplicate_count,
        complex_files_count,
        total_files: files.len(),
        details: HealthDetails {
            dead_code_penalty: dead_penalty,
            duplicate_penalty: dup_penalty,
            complexity_penalty: compl_penalty,
        },
    })
}

/// Print health report to stdout
pub fn print_health_report(report: &HealthReport) {
    use colored::*;

    println!("{}", "Codebase Health".cyan().bold());
    println!("{}", "─".repeat(30).cyan());
    println!();

    let score_color = if report.score >= 80 {
        "green"
    } else if report.score >= 60 {
        "yellow"
    } else {
        "red"
    };

    println!(
        "{} {}",
        "HEALTH SCORE:".cyan().bold(),
        format!("{}/100", report.score).color(score_color).bold()
    );
    println!(
        "├─ Dead code:    {} items ({} pts)",
        report.dead_code_count.to_string().yellow(),
        report.details.dead_code_penalty
    );
    println!(
        "├─ Duplicates:   {} blocks ({} pts)",
        report.duplicate_count.to_string().yellow(),
        report.details.duplicate_penalty
    );
    println!(
        "└─ Complexity:   {} files > {} CC ({} pts)",
        report.complex_files_count.to_string().yellow(),
        COMPLEXITY_THRESHOLD,
        report.details.complexity_penalty
    );
    println!();
    println!("{} {} files analyzed", "•".dimmed(), report.total_files);
    println!();
    println!("{}", "Run with --format json for machine output.".dimmed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_health_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let report = scan_health(dir.path(), None, None).unwrap();
        assert_eq!(report.score, 100);
        assert_eq!(report.dead_code_count, 0);
    }

    #[test]
    fn test_scan_health_with_code() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() { let x = 1; }").unwrap();
        let report = scan_health(dir.path(), Some(&["rs".to_string()]), None).unwrap();
        assert!(report.total_files >= 1);
    }
}
