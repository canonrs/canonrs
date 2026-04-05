//! gen_showcase.rs — gera src/generated/showcase.json
//! SOURCE: canonrs-server/src/ui/*/builder.md

use std::path::Path;
use std::fs;
use crate::build::types::ShowcaseEntry;
use crate::build::parsers::extract_builder_field;

fn extract_section(content: &str, section: &str) -> String {
    let marker = format!("## {}", section);
    if let Some(start) = content.find(&marker) {
        let rest = &content[start + marker.len()..];
        let end = rest.find("\n## ").unwrap_or(rest.len());
        rest[..end].trim().to_string()
    } else {
        String::new()
    }
}

pub(crate) fn generate_showcase(ui_dir: &Path, out_path: &Path) {
    let mut entries: Vec<ShowcaseEntry> = vec![];

    let dir_entries = match fs::read_dir(ui_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in dir_entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let builder_file = path.join("builder.md");
        let content = match fs::read_to_string(&builder_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let pain = match extract_builder_field(&content, "pain") {
            Some(v) => v,
            None => continue,
        };
        let id          = extract_builder_field(&content, "id").unwrap_or_default();
        let label       = extract_builder_field(&content, "label").unwrap_or_default();
        let category    = extract_builder_field(&content, "category").unwrap_or_default();
        let description = extract_builder_field(&content, "description").unwrap_or_default();
        let keywords    = extract_builder_field(&content, "keywords").unwrap_or_default();
        let promise     = extract_builder_field(&content, "promise").unwrap_or_default();
        let why         = extract_builder_field(&content, "why").unwrap_or_default();
        let before      = extract_section(&content, "before");
        let after       = extract_section(&content, "after");
        let rules       = extract_builder_field(&content, "rules")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let use_cases   = extract_builder_field(&content, "use_cases")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let related     = extract_builder_field(&content, "related")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let badges      = extract_builder_field(&content, "badges")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let pillar      = extract_builder_field(&content, "pillar").unwrap_or_default();

        // source files — primitive em canonrs-core/src/primitives/
        let primitives_dir = ui_dir.parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("canonrs-core/src/primitives"))
            .unwrap_or_default();
        let primitive_file = primitives_dir.join(format!("{}.rs", id.replace('-', "_")));
        let primitive_src = std::fs::read_to_string(&primitive_file).unwrap_or_default();

        let ui_src = path.join(format!("{}_ui.rs", id.replace('-', "_")));
        let ui_src = std::fs::read_to_string(&ui_src).unwrap_or_default();

        let island_src = path.join(format!("{}_island.rs", id.replace('-', "_")));
        let island_src = std::fs::read_to_string(&island_src).unwrap_or_default();

        entries.push(ShowcaseEntry {
            id, label, category, description, keywords,
            pain, promise, why, before, after,
            rules, use_cases, related,
            badges, pillar,
            primitive_src, ui_src, island_src,
        });
    }

    entries.sort_by(|a, b| a.id.cmp(&b.id));

    let json = serde_json::to_string_pretty(&entries)
        .expect("showcase json serialize failed");

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(out_path, json).expect("showcase json write failed");
}
