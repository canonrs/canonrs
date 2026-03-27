//! Resolve tipos Rust para PropKind

#[derive(Debug)]
pub enum PropKind {
    StringT,
    Bool,
    Number,
    Children,
    Enum(Vec<String>),
}

/// Caminho base dos primitivos — relativo ao build.rs do canonrs-core
const PRIMITIVES_DIR: &str = "src/primitives";

pub fn resolve_type(ty: &str, name: &str, full_content: &str) -> PropKind {
    let inner = unwrap_option(ty);
    let clean = inner
        .replace("impl Into<String>", "String")
        .replace("impl IntoView", "String");
    let clean = clean.trim();

    if clean == "bool"   { return PropKind::Bool; }
    if clean == "String" || clean.contains("&str") { return PropKind::StringT; }
    if matches!(clean, "f64" | "f32" | "i32" | "i64" | "u32" | "u64" | "usize") {
        return PropKind::Number;
    }
    if clean == "Children" { return PropKind::Children; }

    // Tenta resolver localmente
    let variants = extract_enum_variants(full_content, clean);
    if !variants.is_empty() { return PropKind::Enum(variants); }

    // Tenta resolver em primitivos importados via use canonrs_core::primitives
    if is_imported_from_primitives(full_content, clean) {
        let variants = search_enum_in_primitives(clean);
        if !variants.is_empty() { return PropKind::Enum(variants); }
    }

    // Fallback semântico por nome
    if name.contains("class") || name.contains("label") || name.contains("placeholder")
        || name.contains("id")   || name.contains("aria")  || name.contains("href")
        || name.contains("src")  || name.contains("alt")   || name.contains("title") {
        return PropKind::StringT;
    }

    PropKind::StringT
}

fn extract_enum_variants(content: &str, enum_name: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let needle  = format!("pub enum {} {{", enum_name);
    let needle2 = format!("enum {} {{", enum_name);
    let start   = content.find(&needle).or_else(|| content.find(&needle2));
    let start   = match start { Some(s) => s, None => return variants };
    let bs      = match content[start..].find('{') { Some(b) => start + b + 1, None => return variants };
    let be      = match content[bs..].find('}')    { Some(e) => bs + e,        None => return variants };
    for line in content[bs..be].lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") || t.starts_with('#') { continue; }
        // Suporta múltiplos variants por linha: Primary, Solid, Secondary,
        for part in t.split(',') {
            let v = part.trim().trim_end_matches(',');
            if v.is_empty() || v.starts_with("//") { continue; }
            let variant = v.split('(').next().unwrap_or(v).trim();
            if variant.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                variants.push(pascal_to_kebab(variant));
            }
        }
    }
    variants
}

pub fn pascal_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 { result.push('-'); }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

fn unwrap_option(ty: &str) -> &str {
    let t = ty.trim();
    if t.starts_with("Option<") && t.ends_with('>') { &t[7..t.len()-1] } else { t }
}

fn is_imported_from_primitives(content: &str, enum_name: &str) -> bool {
    // Verifica se o enum aparece num bloco use canonrs_core::primitives
    let mut in_use_block = false;
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("use canonrs_core::primitives") { in_use_block = true; }
        if in_use_block {
            if t.contains(enum_name) { return true; }
            if t.contains(';') { in_use_block = false; }
        }
    }
    false
}

fn search_enum_in_primitives(enum_name: &str) -> Vec<String> {
    use std::fs;
    let dir = std::path::Path::new(PRIMITIVES_DIR);
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return vec![],
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let variants = extract_enum_variants(&content, enum_name);
        if !variants.is_empty() { return variants; }
    }
    vec![]
}
