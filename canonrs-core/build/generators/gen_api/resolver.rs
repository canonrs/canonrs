//! Resolve tipos Rust para PropKind


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

    // Regra: API é gerada lendo APENAS o boundary — zero busca externa
    let variants = extract_enum_variants(full_content, clean);
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

/// Coleta todos os tipos importados de canonrs_core::primitives no boundary
/// Suporta: `pub use canonrs_core::primitives::X` e `use canonrs_core::primitives::{X, Y}`
pub(crate) fn extract_pub_use_types(content: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut multiline = String::new();
    let mut collecting = false;

    for line in content.lines() {
        let t = line.trim();

        // inicia coleta de import multiline de primitives
        if (t.starts_with("pub use canonrs_core::primitives") ||
            t.starts_with("use canonrs_core::primitives")) && !collecting {
            if t.ends_with(';') {
                // single line
                extract_from_import_line(t, &mut types);
            } else {
                // multiline: `use canonrs_core::primitives::{`
                collecting = true;
                multiline = t.to_string();
            }
            continue;
        }

        if collecting {
            multiline.push(' ');
            multiline.push_str(t);
            if t.contains(';') {
                collecting = false;
                extract_from_import_line(&multiline, &mut types);
                multiline.clear();
            }
        }
    }

    // filtra só nomes que parecem enums (PascalCase)
    types.retain(|t| t.chars().next().map(|c| c.is_uppercase()).unwrap_or(false));
    types.dedup();
    types
}

fn extract_from_import_line(line: &str, out: &mut Vec<String>) {
    if line.contains('{') {
        if let (Some(s), Some(e)) = (line.find('{'), line.find('}')) {
            for item in line[s+1..e].split(',') {
                let name = item.trim().trim_end_matches(';').to_string();
                if !name.is_empty() { out.push(name); }
            }
        }
    } else {
        // single: use path::X;
        let clean = line.trim_end_matches(';');
        if let Some(last) = clean.split("::").last() {
            out.push(last.trim().to_string());
        }
    }
}
