//! Structural symbol find: definitions, references, callers
//!
//! Combines extract, callgraph, and search for "grep that understands code".

use crate::callgraph::build_call_graph;
use crate::extract::{extract_classes, extract_functions};
use crate::parser::read_file_content;
use crate::search::list_files;
use crate::types::SearchOptions;
use crate::search::search_code;
use serde::Serialize;
use std::path::Path;

/// A single find result (definition, reference, or call site)
#[derive(Debug, Clone, Serialize)]
pub struct FindResult {
    pub kind: String,
    pub file: String,
    pub line: usize,
    pub content: String,
    pub symbol: String,
}

/// Result of find_symbol
#[derive(Debug, Clone, Serialize)]
pub struct FindReport {
    pub symbol: String,
    pub definitions: Vec<FindResult>,
    pub references: Vec<FindResult>,
    pub callers: Vec<FindResult>,
}

/// Find symbol: definitions, references, and callers
pub fn find_symbol(
    symbol: &str,
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
    find_type: FindType,
) -> Result<FindReport, Box<dyn std::error::Error>> {
    let mut definitions = Vec::new();
    let mut references = Vec::new();
    let mut callers = Vec::new();

    let files = list_files(path, extensions, exclude)?;

    if find_type == FindType::Definition || find_type == FindType::All {
        for file in &files {
            let content = read_file_content(&file.path);
            let path_str = file.path.as_str();

            for (name, line) in extract_functions(&content, &path_str) {
                if name == symbol {
                    let line_content = content.lines().nth(line.saturating_sub(1)).unwrap_or("").trim();
                    definitions.push(FindResult {
                        kind: "FUNCTION".to_string(),
                        file: path_str.to_string(),
                        line,
                        content: line_content.to_string(),
                        symbol: symbol.to_string(),
                    });
                }
            }
            for (name, line) in extract_classes(&content, &path_str) {
                if name == symbol {
                    let line_content = content.lines().nth(line.saturating_sub(1)).unwrap_or("").trim();
                    definitions.push(FindResult {
                        kind: "CLASS".to_string(),
                        file: path_str.to_string(),
                        line,
                        content: line_content.to_string(),
                        symbol: symbol.to_string(),
                    });
                }
            }
        }
    }

    if find_type == FindType::References || find_type == FindType::All {
        let pattern = format!(r"\b{}\b", regex::escape(symbol));
        let options = SearchOptions {
            extensions: extensions.map(|s| s.to_vec()),
            ignore_case: false,
            fuzzy: false,
            fuzzy_threshold: 0.6,
            max_results: 1000,
            exclude: exclude.map(|s| s.to_vec()),
            rank: false,
            cache: false,
            semantic: false,
            benchmark: false,
            vs_grep: false,
        };
        let results = search_code(&pattern, path, &options)?;
        for r in results {
            let is_definition = definitions
                .iter()
                .any(|d| d.file == r.file && d.line == r.line_number);
            if !is_definition {
                references.push(FindResult {
                    kind: "REFERENCE".to_string(),
                    file: r.file.clone(),
                    line: r.line_number,
                    content: r.content.trim().to_string(),
                    symbol: symbol.to_string(),
                });
            }
        }
    }

    if find_type == FindType::Callers || find_type == FindType::All {
        let graph = build_call_graph(path, extensions, exclude)?;
        for edge in &graph.edges {
            if edge.callee == symbol {
                let file = graph
                    .nodes
                    .get(&edge.caller)
                    .map(|n| n.file_path.clone())
                    .unwrap_or_default();
                if !file.is_empty() {
                    let content = std::fs::read_to_string(&file).unwrap_or_default();
                    let line_content = content
                        .lines()
                        .nth(edge.call_site_line.saturating_sub(1))
                        .unwrap_or("")
                        .trim();
                    callers.push(FindResult {
                        kind: "CALL".to_string(),
                        file,
                        line: edge.call_site_line,
                        content: line_content.to_string(),
                        symbol: format!("{} calls {}", edge.caller, symbol),
                    });
                }
            }
        }
    }

    Ok(FindReport {
        symbol: symbol.to_string(),
        definitions,
        references,
        callers,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindType {
    Definition,
    References,
    Callers,
    All,
}

impl FindType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "definition" | "def" | "definitions" => FindType::Definition,
            "references" | "ref" | "refs" => FindType::References,
            "callers" | "calls" => FindType::Callers,
            _ => FindType::All,
        }
    }
}

/// Print find report to stdout
pub fn print_find_report(report: &FindReport, find_type: FindType) {
    use colored::*;

    println!("{}", format!("find '{}'", report.symbol).cyan().bold());
    println!("{}", "─".repeat(40).cyan());
    println!();

    if (find_type == FindType::Definition || find_type == FindType::All) && !report.definitions.is_empty() {
        println!("{}", "DEFINITIONS".green().bold());
        for d in &report.definitions {
            println!("  {} {}:{}", d.file.cyan(), d.line.to_string().yellow(), d.content);
        }
        println!();
    }

    if (find_type == FindType::Callers || find_type == FindType::All) && !report.callers.is_empty() {
        println!("{}", "CALLERS".green().bold());
        for c in &report.callers {
            println!("  {} {}:{}", c.file.cyan(), c.line.to_string().yellow(), c.content);
        }
        println!();
    }

    if (find_type == FindType::References || find_type == FindType::All) && !report.references.is_empty() {
        println!("{}", "REFERENCES".green().bold());
        for r in &report.references {
            println!("  {} {}:{}", r.file.cyan(), r.line.to_string().yellow(), r.content);
        }
        println!();
    }

    if report.definitions.is_empty() && report.callers.is_empty() && report.references.is_empty() {
        println!("{}", "No matches found.".dimmed());
    }
}
