//! gen_showcase.rs — gera src/generated/showcase.json
//! SOURCE: canonrs-server/src/ui/*/_ui.rs @canon-showcase-* fields

use std::path::Path;
use std::fs;
use crate::build::types::ShowcaseEntry;
use crate::build::parsers::extract_canon_field;

pub(crate) fn generate_showcase(ui_dir: &Path, out_path: &Path) {
    let mut entries: Vec<ShowcaseEntry> = vec![];

    let dir_entries = match fs::read_dir(ui_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in dir_entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let dir_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let ui_file = path.join(format!("{}_ui.rs", dir_name));
        let content = match fs::read_to_string(&ui_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if !content.contains("@canon-id:") { continue; }
        // só gera se tiver @canon-pain (sinal que foi preenchido)
        let pain = match extract_canon_field(&content, "canon-pain") {
            Some(v) => v,
            None => continue,
        };
        let id          = extract_canon_field(&content, "canon-id").unwrap_or_default();
        let label       = extract_canon_field(&content, "canon-label").unwrap_or_default();
        let category    = extract_canon_field(&content, "canon-category").unwrap_or_default();
        let description = extract_canon_field(&content, "canon-description").unwrap_or_default();
        let keywords    = extract_canon_field(&content, "canon-keywords").unwrap_or_default();
        let promise     = extract_canon_field(&content, "canon-promise").unwrap_or_default();
        let why         = extract_canon_field(&content, "canon-why").unwrap_or_default();
        let before      = extract_canon_field(&content, "canon-before").unwrap_or_default();
        let after       = extract_canon_field(&content, "canon-after").unwrap_or_default();
        let rules       = extract_canon_field(&content, "canon-rules")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let use_cases   = extract_canon_field(&content, "canon-use-cases")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let related     = extract_canon_field(&content, "canon-related")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();

        entries.push(ShowcaseEntry {
            id, label, category, description, keywords,
            pain, promise, why, before, after,
            rules, use_cases, related,
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
