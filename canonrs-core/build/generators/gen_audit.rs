//! Generator: SSOT_AUDIT.md

use std::fs;
use std::collections::HashMap;
use super::types::*;

pub(crate) fn generate_audit(
    primitives: &HashMap<String, PrimitiveInfo>,
    semantic:   &HashMap<String, SemanticEntry>,
    blocks:     &[BlockInfo],
) {
    let mut missing_sem: Vec<&String> = primitives.keys()
        .filter(|id| !semantic.contains_key(*id)).collect();
    missing_sem.sort();

    let mut orphan_sem: Vec<&String> = semantic.keys()
        .filter(|id| !primitives.contains_key(*id)).collect();
    orphan_sem.sort();

    let mut report = String::from("# CanonRS SSOT Audit\n\n## Components\n");
    report.push_str(&format!("- Primitives: {}\n", primitives.len()));
    report.push_str(&format!("- Semantic (components.toml): {}\n", semantic.len()));
    report.push_str(&format!("- Complete: {}\n", primitives.keys().filter(|id| semantic.contains_key(*id)).count()));
    report.push_str(&format!("- Missing semantic: {}\n", missing_sem.len()));
    report.push_str(&format!("- Orphan semantic: {}\n\n", orphan_sem.len()));

    if !missing_sem.is_empty() {
        report.push_str("### Missing semantic\n");
        for id in &missing_sem { report.push_str(&format!("- `{}`\n", id)); }
    }
    if !orphan_sem.is_empty() {
        report.push_str("\n### Orphan semantic\n");
        for id in &orphan_sem { report.push_str(&format!("- `{}`\n", id)); }
    }

    let block_count  = blocks.iter().filter(|b| b.kind == "block").count();
    let layout_count = blocks.iter().filter(|b| b.kind == "layout").count();
    report.push_str(&format!("\n## Blocks\n- Total: {}\n\n## Layouts\n- Total: {}\n\n", block_count, layout_count));

    report.push_str("### Block list\n");
    let mut sorted = blocks.to_vec();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for b in &sorted {
        report.push_str(&format!("- `{}` ({}) regions: [{}]\n", b.id, b.kind, b.regions.join(", ")));
    }

    fs::write("SSOT_AUDIT.md", report).unwrap();
}
