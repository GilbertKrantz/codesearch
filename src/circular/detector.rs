//! Core circular call detection logic

use super::types::CircularCall;
use crate::extract::{extract_function_calls, extract_functions};
use crate::parser::read_file_content;
use crate::search::list_files;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Find all circular call chains in the codebase.
pub fn find_circular_calls(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<CircularCall>, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;

    if files.is_empty() {
        return Ok(Vec::new());
    }

    let mut call_graph: HashMap<String, (String, HashSet<String>)> = HashMap::new();
    let mut all_functions: HashSet<String> = HashSet::new();

    for file in &files {
        let content = read_file_content(&file.path);
        let functions = extract_functions(&content, &file.path);

        for (func_name, line_num) in functions {
            all_functions.insert(func_name.clone());
            let calls = extract_calls_from_body(&content, line_num);
            call_graph.insert(func_name, (file.path.clone(), calls));
        }
    }

    let mut cycles: Vec<CircularCall> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut rec_stack: HashSet<String> = HashSet::new();
    let mut path_stack: Vec<String> = Vec::new();

    for func in &all_functions {
        if !visited.contains(func) {
            find_cycles_dfs(
                func,
                &call_graph,
                &mut visited,
                &mut rec_stack,
                &mut path_stack,
                &mut cycles,
            );
        }
    }

    Ok(deduplicate_cycles(cycles))
}

fn extract_calls_from_body(content: &str, start_line: usize) -> HashSet<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut calls = HashSet::new();
    let mut brace_count = 0;
    let mut in_body = false;
    let mut started = false;

    let start_idx = start_line.saturating_sub(1);

    for j in start_idx..lines.len().min(start_idx + 200) {
        let body_line = lines[j];

        for c in body_line.chars() {
            if c == '{' {
                brace_count += 1;
                in_body = true;
                started = true;
            } else if c == '}' {
                brace_count -= 1;
            }
        }

        if !started && body_line.trim().ends_with(':') {
            in_body = true;
            started = true;
        }

        if in_body {
            calls.extend(extract_function_calls(body_line));
        }

        if started && brace_count == 0 && in_body {
            break;
        }

        if started
            && !body_line.starts_with(' ')
            && !body_line.starts_with('\t')
            && !body_line.is_empty()
            && j > start_idx
        {
            break;
        }
    }

    calls
}

/// DFS to find cycles in the call graph
pub fn find_cycles_dfs(
    node: &str,
    graph: &HashMap<String, (String, HashSet<String>)>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
    cycles: &mut Vec<CircularCall>,
) {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some((_, calls)) = graph.get(node) {
        for called in calls {
            if graph.contains_key(called) {
                if !visited.contains(called) {
                    find_cycles_dfs(called, graph, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(called) {
                    let cycle_start = path.iter().position(|x| x == called).unwrap();
                    let cycle_chain: Vec<String> = path[cycle_start..].to_vec();
                    let files: Vec<String> = cycle_chain
                        .iter()
                        .filter_map(|f| graph.get(f).map(|(file, _)| file.clone()))
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect();

                    cycles.push(CircularCall {
                        chain: cycle_chain,
                        files,
                    });
                }
            }
        }
    }

    path.pop();
    rec_stack.remove(node);
}

/// Deduplicate cycles (same cycle from different starting nodes)
pub fn deduplicate_cycles(cycles: Vec<CircularCall>) -> Vec<CircularCall> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut unique = Vec::new();

    for cycle in cycles {
        let mut sorted = cycle.chain.clone();
        sorted.sort();
        let key = sorted.join(",");

        if !seen.contains(&key) {
            seen.insert(key);
            unique.push(cycle);
        }
    }

    unique
}

/// Format a cycle chain for display
pub fn format_cycle(chain: &[String]) -> String {
    if chain.is_empty() {
        return String::new();
    }
    let mut result = chain.join(" -> ");
    result.push_str(" -> ");
    result.push_str(&chain[0]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

    #[test]
    fn test_find_circular_calls_with_temp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rs");
        fs::write(&path, "fn a() { b(); }\nfn b() { a(); }").unwrap();
        let parent = path.parent().unwrap();

        let cycles = find_circular_calls(parent, Some(&["rs".to_string()]), None).unwrap();
        assert!(!cycles.is_empty(), "should find at least one cycle");
        let ab_cycle = cycles
            .iter()
            .find(|c| c.chain.contains(&"a".to_string()) && c.chain.contains(&"b".to_string()));
        assert!(ab_cycle.is_some(), "should find a<->b cycle");
    }
}
