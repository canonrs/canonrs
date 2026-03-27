//! Parsers: primitives, semantic (components.toml), blocks/layouts

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use super::types::*;
use super::utils::{pascal_to_kebab};

pub fn parse_primitives(dir: &Path) -> HashMap<String, PrimitiveInfo> {
    let mut map = HashMap::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return map,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        if path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") { continue; }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if let Some(info) = parse_primitive_file(&content) {
            map.insert(info.id.clone(), info);
        }
    }
    map
}

fn parse_primitive_file(content: &str) -> Option<PrimitiveInfo> {
    let component_name = extract_quoted_attr(content, "data-rs-component")?;
    let id             = pascal_to_kebab(&component_name);
    let behavior       = extract_quoted_attr(content, "data-rs-behavior")
        .unwrap_or_else(|| "unknown".to_string());
    let variants       = extract_variants(content);
    Some(PrimitiveInfo { id, component_name, behavior, variants })
}

fn extract_quoted_attr(content: &str, attr: &str) -> Option<String> {
    let needle = format!("{}=\"", attr);
    let start  = content.find(&needle)? + needle.len();
    let end    = content[start..].find('"')? + start;
    Some(content[start..end].to_string())
}

fn extract_variants(content: &str) -> Vec<VariantInfo> {
    let mut variants = vec![];
    let mut in_enum  = false;
    let mut name     = String::new();
    let mut values   = vec![];
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("pub enum ") && !t.contains("State") && !t.contains("Type") {
            in_enum = true;
            name    = t.trim_start_matches("pub enum ").trim_end_matches(" {").to_string();
            values.clear();
        } else if in_enum && t == "}" {
            if !name.is_empty() && !values.is_empty() {
                variants.push(VariantInfo { enum_name: name.clone(), values: values.clone() });
            }
            in_enum = false;
        } else if in_enum && !t.starts_with("//") && !t.starts_with('#') {
            let v = t.trim_end_matches(',');
            if !v.is_empty() && v.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                values.push(v.to_string());
            }
        }
    }
    variants
}



pub fn parse_blocks_and_layouts(blocks_dir: &Path, layouts_dir: &Path) -> Vec<BlockInfo> {
    let mut result = vec![];
    for (dir, kind) in &[(blocks_dir, "block"), (layouts_dir, "layout")] {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() { continue; }
            let files = match fs::read_dir(&path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            for file in files.flatten() {
                let fpath = file.path();
                let fname = fpath.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !fname.ends_with(".rs") || fname == "mod.rs"
                    || fname.contains("example") || fname.contains("mock") { continue; }
                let content = match fs::read_to_string(&fpath) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                if !content.contains("@canon-id:") { continue; }
                if let Some(info) = parse_canon_header(&content, kind) {
                    result.push(info);
                }
            }
        }
    }
    result
}

fn parse_canon_header(content: &str, kind: &str) -> Option<BlockInfo> {
    let id        = extract_canon_field(content, "canon-id")?;
    let category  = extract_canon_field(content, "canon-category").unwrap_or_else(|| "layout".to_string());
    let variant   = extract_canon_field(content, "canon-variant").unwrap_or_else(|| "structure".to_string());
    let container = extract_canon_field(content, "canon-container").map(|v| v == "true").unwrap_or(false);
    let regions   = extract_canon_field(content, "canon-regions")
        .map(|r| r.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    let label       = extract_canon_field(content, "canon-label");
    let description = extract_canon_field(content, "canon-description");
    let tags        = extract_canon_field(content, "canon-tags")
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    Some(BlockInfo { id, kind: kind.to_string(), category, variant, container, regions, label, description, tags })
}

pub fn extract_canon_field(content: &str, field: &str) -> Option<String> {
    let needle = format!("//! @{}:", field);
    let line   = content.lines().find(|l| l.contains(&needle))?;
    Some(line.splitn(2, &needle).nth(1)?.trim().to_string())
}

pub fn parse_block_props(content: &str) -> Vec<PropInfo> {
    let mut props = vec![];
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with("//! @canon-prop:") { continue; }
        let val = t.trim_start_matches("//! @canon-prop:").trim();
        let parts: Vec<&str> = val.splitn(6, '|').map(|p| p.trim()).collect();
        if parts.len() < 4 { continue; }
        let key     = parts[0].to_string();
        let field   = parts[1].to_string();
        let default = if parts[2].is_empty() { None } else { Some(parts[2].to_string()) };
        let scope   = parts[3].to_string();
        let css     = if parts.len() > 4 && !parts[4].is_empty() { Some(parts[4].to_string()) } else { None };
        let label   = key.replace('-', " ")
            .split_whitespace()
            .map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() })
            .collect::<Vec<_>>()
            .join(" ");
        props.push(PropInfo { key, label, field, default, scope, css });
    }
    props
}

pub fn parse_block_presets(content: &str) -> Vec<PresetInfo> {
    let mut presets = vec![];
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with("//! @canon-preset:") { continue; }
        let val = t.trim_start_matches("//! @canon-preset:").trim();
        let parts: Vec<&str> = val.splitn(2, '|').map(|p| p.trim()).collect();
        if parts.len() < 2 { continue; }
        let label = parts[0].to_string();
        let props: Vec<(String, String)> = split_outside_parens(parts[1]).into_iter()
            .filter_map(|p| {
                let kv: Vec<&str> = p.splitn(2, '=').collect();
                if kv.len() == 2 { Some((kv[0].trim().to_string(), kv[1].trim().to_string())) }
                else { None }
            })
            .collect();
        presets.push(PresetInfo { label, props });
    }
    presets
}

pub fn parse_slot_descriptions(content: &str) -> Vec<(String, String)> {
    let needle = "//! @canon-slot-descriptions:";
    let line = match content.lines().find(|l| l.contains(needle)) {
        Some(l) => l,
        None => return vec![],
    };
    let val = line.splitn(2, needle).nth(1).unwrap_or("").trim();
    val.split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.splitn(2, ':').collect();
            if parts.len() == 2 { Some((parts[0].trim().to_string(), parts[1].trim().to_string())) }
            else { None }
        })
        .collect()
}


pub fn parse_slot_accepts(content: &str) -> Vec<(String, String)> {
    let needle = "//! @canon-slot-accepts:";
    let line = match content.lines().find(|l| l.contains(needle)) {
        Some(l) => l,
        None => return vec![],
    };
    let val = line.splitn(2, needle).nth(1).unwrap_or("").trim();
    val.split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.splitn(2, '=').collect();
            if parts.len() == 2 { Some((parts[0].trim().to_string(), parts[1].trim().to_string())) }
            else { None }
        })
        .collect()
}

pub fn parse_ui_components_semantic(ui_dir: &Path) -> HashMap<String, SemanticEntry> {
    let mut map = HashMap::new();
    let entries = match fs::read_dir(ui_dir) {
        Ok(e) => e,
        Err(_) => return map,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let dir_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        // Encontra o *_ui.rs principal (mesmo nome do diretório)
        let ui_file = path.join(format!("{}_ui.rs", dir_name));
        let content = match fs::read_to_string(&ui_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // Só processa se tem @canon-id
        let id = match extract_canon_field(&content, "canon-id") {
            Some(v) => v,
            None => continue,
        };
        let label       = extract_canon_field(&content, "canon-label").unwrap_or_default();
        let description = extract_canon_field(&content, "canon-description").unwrap_or_default();
        let family      = extract_canon_field(&content, "canon-family").unwrap_or_default();
        let intent      = extract_canon_field(&content, "canon-intent").unwrap_or_default();
        let catalog_category = extract_canon_field(&content, "canon-category").unwrap_or_default();
        let composable  = extract_canon_field(&content, "canon-composable")
            .map(|v| v == "true")
            .unwrap_or(false);
        let capabilities = extract_canon_field(&content, "canon-capabilities")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let catalog_tags = extract_canon_field(&content, "canon-tags")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let required_parts = extract_canon_field(&content, "canon-required-parts")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let optional_parts = extract_canon_field(&content, "canon-optional-parts")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        map.insert(id.clone(), SemanticEntry {
            id,
            label,
            description,
            family,
            intent,
            capabilities,
            catalog_tags,
            catalog_category,
            required_parts,
            optional_parts,
            composable,
            requires_config: false,
        });
    }
    map
}

fn split_outside_parens(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth  = 0usize;
    let mut start  = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => { if depth > 0 { depth -= 1; } }
            ',' if depth == 0 => {
                result.push(s[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < s.len() { result.push(s[start..].trim()); }
    result
}
