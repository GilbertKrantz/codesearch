//! Dead code detection logic

use super::detectors::{
    detect_dead_code_patterns, detect_empty_functions, detect_todo_fixme,
    detect_unreachable_code, detect_unused_variables,
};
use super::helpers::is_special_function;
use super::types::DeadCodeItem;
use crate::extract::{extract_classes, extract_functions, extract_identifier_references};
use crate::parser::read_file_content;
use crate::search::list_files;
use std::collections::HashMap;
use std::path::Path;

/// Find dead code items in the codebase.
pub fn find_dead_code(
    path: &Path,
    extensions: Option<&[String]>,
    exclude: Option<&[String]>,
) -> Result<Vec<DeadCodeItem>, Box<dyn std::error::Error>> {
    let files = list_files(path, extensions, exclude)?;

    if files.is_empty() {
        return Ok(Vec::new());
    }

    let mut dead_code_items: Vec<DeadCodeItem> = Vec::new();
    let mut all_definitions: HashMap<String, (String, usize, String)> = HashMap::new();
    let mut all_references: HashMap<String, usize> = HashMap::new();

    for file in &files {
        let content = read_file_content(&file.path);

        for (name, line_num) in extract_functions(&content, &file.path) {
            if !is_special_function(&name) {
                all_definitions.insert(
                    name.clone(),
                    (file.path.clone(), line_num, "function".to_string()),
                );
            }
        }

        for (name, line_num) in extract_classes(&content, &file.path) {
            all_definitions.insert(
                name.clone(),
                (file.path.clone(), line_num, "class/struct".to_string()),
            );
        }

        for ref_name in extract_identifier_references(&content) {
            *all_references.entry(ref_name).or_insert(0) += 1;
        }
    }

    for (name, (file, line, item_type)) in &all_definitions {
        let ref_count = all_references.get(name).copied().unwrap_or(0);

        if ref_count <= 1 {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Only defined, never used elsewhere".to_string(),
            });
        } else if ref_count == 2 && item_type == "function" {
            dead_code_items.push(DeadCodeItem {
                file: file.clone(),
                line_number: *line,
                item_type: item_type.clone(),
                name: name.clone(),
                reason: "Used only once - consider inlining".to_string(),
            });
        }
    }

    for file in &files {
        let content = read_file_content(&file.path);
        detect_dead_code_patterns(&file.path, &content, &mut dead_code_items);
        detect_unused_variables(&file.path, &content, &mut dead_code_items);
        detect_unreachable_code(&file.path, &content, &mut dead_code_items);
        detect_empty_functions(&file.path, &content, &mut dead_code_items);
        detect_todo_fixme(&file.path, &content, &mut dead_code_items);
    }

    dead_code_items.sort_by(|a, b| {
        a.file.cmp(&b.file).then(a.line_number.cmp(&b.line_number))
    });

    Ok(dead_code_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_dead_code_with_temp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.rs");
        fs::write(&path, "fn orphan_fn() { println!(\"hi\"); }").unwrap();
        let parent = path.parent().unwrap();

        let items = find_dead_code(parent, Some(&["rs".to_string()]), None).unwrap();
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.name == "orphan_fn"));
    }
}
