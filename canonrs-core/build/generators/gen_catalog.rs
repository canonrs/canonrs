//! Generator: catalog.rs

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use super::types::*;
use super::utils::*;
use super::parsers::parse_slot_accepts;

pub(crate) fn generate_catalog(semantic: &HashMap<String, SemanticEntry>, blocks: &[BlockInfo], out_dir: &Path, blocks_dir: &Path, layouts_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("use crate::catalog_types::{CatalogEntry, CatalogCategory, CatalogKind, CatalogAcceptRule, CatalogRegionRule};\n\n");
    code.push_str("pub static CATALOG_GENERATED: &[CatalogEntry] = &[\n");

    // Components — ordenados por id
    let mut ids: Vec<&String> = semantic.keys().collect();
    ids.sort();
    for id in &ids {
        let s = &semantic[*id];
        let cat   = to_catalog_category(&s.catalog_category);
        let tags  = fmt_str_slice(&s.catalog_tags);
        let parts = fmt_str_slice(&s.required_parts.iter()
            .chain(s.optional_parts.iter())
            .cloned().collect::<Vec<_>>());
        code.push_str(&format!(
            "    CatalogEntry {{ id: \"{id}\", label: \"{label}\", description: \"{desc}\", kind: CatalogKind::Component, category: {cat}, tags: &[{tags}], parts: &[{parts}], regions: &[], accepts: {accepts}, region_rules: &[] }},\n",
            id = id, label = s.label, desc = s.description, cat = cat, tags = tags, parts = parts,
            accepts = catalog_accepts_for_component()
        ));
    }

    // Blocks
    for b in blocks.iter().filter(|b| b.kind == "block") {
        let label   = b.label.clone().unwrap_or_else(|| to_title_case(&b.id));
        let desc    = b.description.clone().unwrap_or_else(|| format!("{} block", label));
        let tags    = fmt_str_slice(&if b.tags.is_empty() { vec![b.id.clone()] } else { b.tags.clone() });
        let regions = fmt_str_slice(&b.regions);
        let cat     = to_catalog_category(&b.category);
        let block_accepts      = catalog_accepts_for_block(&b.category);
        let block_file = blocks_dir.join(b.id.replace('-', "_")).join(format!("{}_block.rs", b.id.replace('-', "_")));
        let block_region_rules = catalog_region_rules_from_file(&block_file);
        code.push_str(&format!(
            "    CatalogEntry {{ id: \"block.{id}\", label: \"{label}\", description: \"{desc}\", kind: CatalogKind::Block, category: {cat}, tags: &[{tags}], parts: &[], regions: &[{regions}], accepts: {accepts}, region_rules: {rr} }},\n",
            id = b.id, label = label, desc = desc, cat = cat, tags = tags, regions = regions,
            accepts = block_accepts, rr = block_region_rules
        ));
    }

    // Layouts
    for b in blocks.iter().filter(|b| b.kind == "layout") {
        let label   = b.label.clone().unwrap_or_else(|| to_title_case(&b.id));
        let desc    = b.description.clone().unwrap_or_else(|| format!("{} layout", label));
        let tags    = fmt_str_slice(&if b.tags.is_empty() { vec![b.id.clone()] } else { b.tags.clone() });
        let regions = fmt_str_slice(&b.regions);
        // Tenta com id snake_case, fallback para nome do diretório sem sufixo
        let layout_dir_name = b.id.replace('-', "_").replace("_layout", "");
        let layout_file1 = layouts_dir.join(&layout_dir_name).join(format!("{}_layout.rs", layout_dir_name));
        let layout_file2 = layouts_dir.join(b.id.replace('-', "_")).join(format!("{}_layout.rs", b.id.replace('-', "_")));
        let layout_file = if layout_file1.exists() { layout_file1 } else { layout_file2 };
        let layout_region_rules = catalog_region_rules_from_file(&layout_file);
        code.push_str(&format!(
            "    CatalogEntry {{ id: \"layout.{id}\", label: \"{label}\", description: \"{desc}\", kind: CatalogKind::Layout, category: CatalogCategory::Layout, tags: &[{tags}], parts: &[], regions: &[{regions}], accepts: {accepts}, region_rules: {rr} }},\n",
            id = b.id, label = label, desc = desc, tags = tags, regions = regions,
            accepts = catalog_accepts_for_layout(), rr = layout_region_rules
        ));
    }

    code.push_str("];\n");
    fs::write(out_dir.join("catalog.rs"), code).unwrap();
}

fn fmt_str_slice(items: &[String]) -> String {
    items.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ")
}

fn catalog_accepts_for_component() -> &'static str {
    // Componentes são leaf nodes — não aceitam filhos estruturais
    "&[]"
}

fn catalog_accepts_for_block(category: &str) -> String {
    match category {
        "overlay" => "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]".to_string(),
        "form"    => "&[CatalogAcceptRule::ComponentCategory(CatalogCategory::Form)]".to_string(),
        "data"    => "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]".to_string(),
        _         => "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]".to_string(),
    }
}

fn catalog_accepts_for_layout() -> &'static str {
    "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]"
}

fn slot_accept_to_rule(accept: &str) -> &'static str {
    match accept {
        "Nav"    => "&[CatalogAcceptRule::ComponentCategory(CatalogCategory::Navigation)]",
        "Action" => "&[CatalogAcceptRule::ComponentCategory(CatalogCategory::Action)]",
        "Form"   => "&[CatalogAcceptRule::ComponentCategory(CatalogCategory::Form)]",
        "Any"    => "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]",
        _        => "&[CatalogAcceptRule::AnyComponent, CatalogAcceptRule::AnyBlock]",
    }
}

fn catalog_region_rules_from_file(file_path: &std::path::Path) -> String {
    let content = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return "&[]".to_string(),
    };
    let slot_accepts = parse_slot_accepts(&content);
    if slot_accepts.is_empty() { return "&[]".to_string(); }
    let rules: Vec<String> = slot_accepts.iter().map(|(region, accept)| {
        let rule = slot_accept_to_rule(accept.as_str());
        format!("CatalogRegionRule {{ region: \"{region}\", accepts: {rule} }}", region = region, rule = rule)
    }).collect();
    format!("&[{}]", rules.join(", "))
}
