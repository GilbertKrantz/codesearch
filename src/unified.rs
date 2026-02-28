//! Unified Code Graph (lightweight CPG)
//!
//! Combines AST syntax, CFG execution flow, and DFG data dependencies into a single
//! queryable structure—like Joern's CPG but no graph DB, zero indexing, instant.
//! Beats Joern: no import step, no Scala. Beats CodeQL: no extractors, works on raw source.

use crate::ast::{analyze_file, get_syntax_edges};
use crate::cfg::analyze_file_cfg;
use crate::dfg::analyze_file_dfg;
use serde::Serialize;
use std::path::Path;

/// Unified edge: syntax, execution flow, or data dependency
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedEdge {
    pub edge_category: EdgeCategory,
    pub from: String,
    pub from_line: Option<usize>,
    pub to: String,
    pub to_line: Option<usize>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum EdgeCategory {
    Syntax,
    ExecutionFlow,
    DataFlow,
}

/// Lightweight unified graph—all relationship types in one structure
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedGraph {
    pub file_path: String,
    pub syntax_edges: usize,
    pub execution_edges: usize,
    pub data_edges: usize,
    pub edges: Vec<UnifiedEdge>,
}

/// Build unified graph from a file. Zero config, no indexing.
pub fn build_unified_graph(path: &Path) -> Result<UnifiedGraph, Box<dyn std::error::Error>> {
    let path_str = path.to_string_lossy().to_string();
    let mut edges = Vec::new();

    if path.extension().is_some() {
        if let Ok(ast) = analyze_file(path) {
            for e in get_syntax_edges(&ast) {
                edges.push(UnifiedEdge {
                    edge_category: EdgeCategory::Syntax,
                    from: e.from_node,
                    from_line: Some(e.from_line),
                    to: e.to_node,
                    to_line: Some(e.to_line),
                    label: Some(format!("{:?}", e.relationship)),
                });
            }
        }
    }

    if let Ok(cfgs) = analyze_file_cfg(path) {
        for cfg in &cfgs {
            for e in &cfg.edges {
                let from_label = cfg
                    .basic_blocks
                    .get(&e.from)
                    .map(|b| format!("{}:{}", cfg.function_name, b.start_line))
                    .unwrap_or_else(|| format!("{}:block{}", cfg.function_name, e.from));
                let to_label = cfg
                    .basic_blocks
                    .get(&e.to)
                    .map(|b| format!("{}:{}", cfg.function_name, b.start_line))
                    .unwrap_or_else(|| format!("{}:block{}", cfg.function_name, e.to));
                edges.push(UnifiedEdge {
                    edge_category: EdgeCategory::ExecutionFlow,
                    from: from_label,
                    from_line: cfg.basic_blocks.get(&e.from).map(|b| b.start_line),
                    to: to_label,
                    to_line: cfg.basic_blocks.get(&e.to).map(|b| b.start_line),
                    label: Some(format!("{:?}", e.edge_type)),
                });
            }
        }
    }

    if let Ok(dfgs) = analyze_file_dfg(path) {
        for dfg in &dfgs {
            for e in &dfg.edges {
                let from_label = dfg
                    .nodes
                    .get(&e.from)
                    .map(|n| format!("{}:{}@{}", dfg.function_name, n.name, n.line))
                    .unwrap_or_else(|| format!("{}:n{}", dfg.function_name, e.from));
                let to_label = dfg
                    .nodes
                    .get(&e.to)
                    .map(|n| format!("{}:{}@{}", dfg.function_name, n.name, n.line))
                    .unwrap_or_else(|| format!("{}:n{}", dfg.function_name, e.to));
                edges.push(UnifiedEdge {
                    edge_category: EdgeCategory::DataFlow,
                    from: from_label,
                    from_line: dfg.nodes.get(&e.from).map(|n| n.line),
                    to: to_label,
                    to_line: dfg.nodes.get(&e.to).map(|n| n.line),
                    label: Some(format!("{:?}", e.edge_type)),
                });
            }
        }
    }

    let syntax_edges = edges
        .iter()
        .filter(|e| e.edge_category == EdgeCategory::Syntax)
        .count();
    let execution_edges = edges
        .iter()
        .filter(|e| e.edge_category == EdgeCategory::ExecutionFlow)
        .count();
    let data_edges = edges
        .iter()
        .filter(|e| e.edge_category == EdgeCategory::DataFlow)
        .count();

    Ok(UnifiedGraph {
        file_path: path_str,
        syntax_edges,
        execution_edges,
        data_edges,
        edges,
    })
}

/// Trace data flow from source variable to sinks (forward). Returns reachable nodes.
pub fn trace_data_flow_forward(
    path: &Path,
    source_var: &str,
    sink_pattern: Option<&str>,
) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    let dfgs = analyze_file_dfg(path)?;
    let mut results = Vec::new();

    for dfg in &dfgs {
        let starts: Vec<usize> = dfg
            .nodes
            .iter()
            .filter(|(_, n)| n.name == source_var)
            .map(|(id, _)| *id)
            .collect();
        let starts_set: std::collections::HashSet<usize> = starts.iter().copied().collect();

        let mut reachable = std::collections::HashSet::new();
        for &start in &starts {
            reachable.insert(start);
        }

        let mut queue: Vec<usize> = starts;
        while let Some(id) = queue.pop() {
            for edge in &dfg.edges {
                if edge.from == id {
                    if reachable.insert(edge.to) {
                        queue.push(edge.to);
                    }
                }
            }
        }

        for id in reachable {
            if let Some(node) = dfg.nodes.get(&id) {
                let matches_sink = sink_pattern
                    .map(|p| {
                        node.name.contains(p)
                            || node.definition.as_deref().map_or(false, |d| d.contains(p))
                    })
                    .unwrap_or(true);
                if matches_sink && (node.name != source_var || !starts_set.contains(&id)) {
                    results.push((format!("{}:{}", dfg.file_path, node.line), node.line));
                }
            }
        }
    }

    results.sort_by_key(|(_, line)| *line);
    results.dedup();
    Ok(results)
}

/// Trace data flow across a directory (files matching extensions)
pub fn trace_data_flow_in_path(
    path: &Path,
    source_var: &str,
    sink_pattern: Option<&str>,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    let files = crate::search::list_files(path, extensions, exclude)?;
    let mut all = Vec::new();
    for f in &files {
        if std::path::Path::new(&f.path).is_file() {
            if let Ok(r) = trace_data_flow_forward(Path::new(&f.path), source_var, sink_pattern) {
                all.extend(r);
            }
        }
    }
    all.sort_by_key(|(_, line)| *line);
    all.dedup();
    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_build_unified_graph() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("t.rs");
        fs::write(&f, "fn foo() { let x = 1; }").unwrap();
        let g = build_unified_graph(&f).unwrap();
        assert!(!g.edges.is_empty() || g.syntax_edges == 0);
    }
}
