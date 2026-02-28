//! Basic security pattern detection
//!
//! Lightweight checks for common dangerous patterns—like CodeQL/Joern queries
//! but instant, no indexing, no query language. Beats them on setup time.

use crate::search::list_files;
use regex::Regex;
use serde::Serialize;
use std::path::Path;

/// A suspected security issue
#[derive(Debug, Clone, Serialize)]
pub struct SecurityFinding {
    pub kind: SecurityKind,
    pub file: String,
    pub line: usize,
    pub content: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SecurityKind {
    EvalWithInput,
    ExecWithInput,
    SqlConcat,
    CommandInjection,
    HardcodedSecret,
    UnsafeDeserialize,
    XssSink,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Severity {
    High,
    Medium,
    Low,
}

/// Scan for dangerous security patterns. Returns findings.
pub fn scan_security_patterns(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<SecurityFinding>, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;
    let mut findings = Vec::new();

    let patterns: Vec<(SecurityKind, Severity, &str)> = vec![
        (
            SecurityKind::EvalWithInput,
            Severity::High,
            r"(?i)eval\s*\(\s*(?:req|request|input|params|query|body|user)",
        ),
        (
            SecurityKind::ExecWithInput,
            Severity::High,
            r"(?i)(?:exec|system|popen|shell_exec|passthru)\s*\(\s*(?:req|request|input|params|query|\$_)",
        ),
        (
            SecurityKind::SqlConcat,
            Severity::High,
            r"(?i)(?:execute|query|prepare)\s*\(\s*[^?]*\+|SELECT.*\+.*(?:req|request|input|params)",
        ),
        (
            SecurityKind::CommandInjection,
            Severity::High,
            r"(?i)(?:os\.system|subprocess\.call|exec)\s*\([^)]*(?:input|request|argv)",
        ),
        (
            SecurityKind::UnsafeDeserialize,
            Severity::Medium,
            r"(?i)(?:pickle\.loads|yaml\.load\s*\(|unserialize)\s*\([^)]*\)",
        ),
        (
            SecurityKind::XssSink,
            Severity::Medium,
            r"(?i)innerHTML\s*=|document\.write\s*\(|dangerouslySetInnerHTML",
        ),
        (
            SecurityKind::HardcodedSecret,
            Severity::Low,
            r#"(?i)(?:password|api_key|secret)\s*=\s*['"][^'"]{8,}['"]"#,
        ),
    ];

    for file in &files {
        let content = match std::fs::read_to_string(&file.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (kind, severity, pattern) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                for (line_num, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        let trimmed = line.trim();
                        if trimmed.is_empty()
                            || trimmed.starts_with("//")
                            || trimmed.starts_with("#")
                        {
                            continue;
                        }
                        findings.push(SecurityFinding {
                            kind: kind.clone(),
                            file: file.path.clone(),
                            line: line_num + 1,
                            content: trimmed.to_string(),
                            severity: severity.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(findings)
}

/// Print security findings to stdout
pub fn print_security_report(findings: &[SecurityFinding]) {
    use colored::*;

    if findings.is_empty() {
        println!("{}", "No security issues found.".green());
        return;
    }

    println!("{}", "Security Scan".cyan().bold());
    println!("{}", "─".repeat(50).cyan());
    println!();

    let high: Vec<_> = findings
        .iter()
        .filter(|f| f.severity == Severity::High)
        .collect();
    let medium: Vec<_> = findings
        .iter()
        .filter(|f| f.severity == Severity::Medium)
        .collect();
    let low: Vec<_> = findings
        .iter()
        .filter(|f| f.severity == Severity::Low)
        .collect();

    if !high.is_empty() {
        println!("{}", "HIGH".red().bold());
        for f in &high {
            println!(
                "  {} {}:{} {}",
                "⚠".red(),
                f.file.cyan(),
                f.line.to_string().yellow(),
                f.content
            );
        }
        println!();
    }
    if !medium.is_empty() {
        println!("{}", "MEDIUM".yellow().bold());
        for f in &medium {
            println!(
                "  {} {}:{} {}",
                "▪".yellow(),
                f.file.cyan(),
                f.line.to_string().yellow(),
                f.content
            );
        }
        println!();
    }
    if !low.is_empty() {
        println!("{}", "LOW".dimmed().bold());
        for f in &low {
            println!(
                "  {} {}:{} {}",
                "·".dimmed(),
                f.file.cyan(),
                f.line.to_string().yellow(),
                f.content
            );
        }
    }

    println!();
    println!("{} {} total", "•".dimmed(), findings.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_eval_pattern() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("bad.js");
        std::fs::write(&f, "eval(req.body.input);").unwrap();
        let findings = scan_security_patterns(dir.path(), Some(&["js".to_string()]), None).unwrap();
        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .any(|f| f.kind == SecurityKind::EvalWithInput)
        );
    }
}
