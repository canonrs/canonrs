//! Resolve tipos Rust para PropKind

use std::fs;
use std::path::Path;

#[derive(Debug)]
pub(crate) enum PropKind {
    StringT,
    Bool,
    Number,
    Children,
    Enum(Vec<String>),
}

pub(crate) fn resolve_type(ty: &str, name: &str, full_content: &str) -> PropKind {
    let inner = unwrap_option(ty);
    let clean = inner
        .replace("impl Into<String>", "String")
        .replace("impl IntoView", "String");
    // Extrai nome simples de paths completos: canonrs_core::meta::DisabledState → DisabledState
    let clean = if clean.contains("::") && !clean.starts_with("Option<") {
        clean.split("::").last().unwrap_or(clean.trim())
    } else {
        clean.trim()
    };
    let clean = clean.trim();

    if clean == "bool"   { return PropKind::Bool; }
    if clean == "String" || clean.contains("&str") { return PropKind::StringT; }
    if matches!(clean, "f64" | "f32" | "i32" | "i64" | "u32" | "u64" | "usize") {
        return PropKind::Number;
    }
    if clean == "Children" { return PropKind::Children; }
    if clean == "ChildrenFn" || clean == "Option<ChildrenFn>" { return PropKind::Children; }

    // 1. Tenta resolver localmente no conteudo do boundary
    let variants = extract_enum_variants(full_content, clean);
    if !variants.is_empty() { return PropKind::Enum(variants); }

    // 2. Tenta resolver nos arquivos do core/server
    if is_imported_from_known_crate(full_content, clean) {
        let variants = search_enum_in_all_sources(clean);
        if !variants.is_empty() { return PropKind::Enum(variants); }
    }

    // 3. Tenta busca incondicional (para casos como pub use)
    let variants = search_enum_in_all_sources(clean);
    if !variants.is_empty() { return PropKind::Enum(variants); }

    // Fallback semantico por nome
    if name.contains("class") || name.contains("label") || name.contains("placeholder")
        || name.contains("id")   || name.contains("aria")  || name.contains("href")
        || name.contains("src")  || name.contains("alt")   || name.contains("title") {
        return PropKind::StringT;
    }

    PropKind::StringT
}

pub(crate) fn extract_enum_variants(content: &str, enum_name: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let needle  = format!("pub enum {} {{", enum_name);
    let needle2 = format!("pub(crate) enum {} {{", enum_name);
    let needle3 = format!("enum {} {{", enum_name);
    let start = content.find(&needle)
        .or_else(|| content.find(&needle2))
        .or_else(|| content.find(&needle3));
    let start = match start { Some(s) => s, None => return variants };
    let bs = match content[start..].find('{') { Some(b) => start + b + 1, None => return variants };
    let be = match content[bs..].find('}')    { Some(e) => bs + e,        None => return variants };
    for line in content[bs..be].lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("//") || t.starts_with('#') { continue; }
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

pub(crate) fn pascal_to_kebab(s: &str) -> String {
    match s {
        "One"    => return "1".to_string(),
        "Two"    => return "2".to_string(),
        "Three"  => return "3".to_string(),
        "Four"   => return "4".to_string(),
        "Five"   => return "5".to_string(),
        "Six"    => return "6".to_string(),
        "Seven"  => return "7".to_string(),
        "Eight"  => return "8".to_string(),
        "Nine"   => return "9".to_string(),
        "Ten"    => return "10".to_string(),
        "Eleven" => return "11".to_string(),
        "Twelve" => return "12".to_string(),
        _ => {}
    }
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

fn is_imported_from_known_crate(content: &str, enum_name: &str) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if (t.starts_with("use canonrs_core")
            || t.starts_with("use canonrs::")
            || t.starts_with("pub use canonrs")
            || t.starts_with("use super::"))
            && t.contains(enum_name)
        {
            return true;
        }
    }
    false
}

fn search_enum_in_all_sources(enum_name: &str) -> Vec<String> {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let base = Path::new(&manifest);

    let search_dirs = vec![
        base.join("src/primitives"),
        base.join("src/meta"),
        base.join("src"),
        base.join("../canonrs-server/src/ui"),
        base.join("../canonrs-server/src/blocks"),
        base.join("../canonrs-server/src/layouts"),
    ];

    for dir in &search_dirs {
        if let Ok(variants) = search_dir_for_enum(dir, enum_name) {
            if !variants.is_empty() { return variants; }
        }
    }
    vec![]
}

fn search_dir_for_enum(dir: &Path, enum_name: &str) -> std::io::Result<Vec<String>> {
    if !dir.exists() { return Ok(vec![]); }
    let entries: Vec<_> = fs::read_dir(dir)?.flatten().collect();

    // Primeiro: checar arquivos .rs no diretorio atual
    for entry in &entries {
        let path = entry.path();
        if path.is_dir() { continue; }
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let variants = extract_enum_variants(&content, enum_name);
        if !variants.is_empty() { return Ok(variants); }
    }

    // Depois: buscar em subdiretórios
    for entry in &entries {
        let path = entry.path();
        if !path.is_dir() { continue; }
        if let Ok(v) = search_dir_for_enum(&path, enum_name) {
            if !v.is_empty() { return Ok(v); }
        }
    }

    Ok(vec![])
}
