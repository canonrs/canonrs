//! gen_rules.rs — Gera rules.json, rules_seo.json, rules_llm.md a partir dos .md
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub(crate) struct RuleInfo {
    pub number:   u32,
    pub slug:     String,
    pub title:    String,
    pub status:   String,
    pub severity: String,
    pub scopes:   Vec<String>,
    pub version:  String,
    pub date:     String,
    pub body:     String,
}

pub(crate) fn parse_rules(rules_dir: &Path) -> Vec<RuleInfo> {
    let mut rules = Vec::new();
    let mut entries: Vec<_> = match fs::read_dir(rules_dir) {
        Ok(e) => e.filter_map(|e| e.ok()).collect(),
        Err(_) => return rules,
    };
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") { continue; }
        let fname = path.file_stem().unwrap().to_str().unwrap().to_string();
        if !fname.starts_with("canon-rule-") { continue; }
        let src = match fs::read_to_string(&path) { Ok(c) => c, Err(_) => continue };
        let number = extract_number_from_title(&src)
            .or_else(|| extract_number_from_filename(&fname))
            .unwrap_or(0);
        let title    = extract_title(&src).unwrap_or_else(|| fname.clone());
        let status   = extract_field(&src, "Status").unwrap_or_else(|| "DRAFT".to_string());
        let severity = extract_field(&src, "Severity").unwrap_or_else(|| "LOW".to_string());
        let scopes   = extract_field(&src, "Scope")
            .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_default();
        let version  = extract_field(&src, "Version").unwrap_or_else(|| "1.0.0".to_string());
        let date     = extract_field(&src, "Date").unwrap_or_else(|| "2025-01-01".to_string());
        let body     = extract_body(&src);
        let slug     = fname.trim_start_matches("canon-rule-").to_string();
        rules.push(RuleInfo { number, slug, title, status, severity, scopes, version, date, body });
    }
    rules.sort_by_key(|r| r.number);
    rules
}

fn extract_number_from_title(src: &str) -> Option<u32> {
    let line = src.lines().find(|l| l.starts_with("# Canon Rule"))?;
    let hash_pos = line.find('#')?;
    let rest = &line[hash_pos + 1..];
    let num_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    num_str.parse().ok()
}

fn extract_number_from_filename(fname: &str) -> Option<u32> {
    let after = fname.strip_prefix("canon-rule-")?;
    let num_str: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
    num_str.parse().ok()
}

fn extract_title(src: &str) -> Option<String> {
    let line = src.lines().find(|l| l.starts_with("# Canon Rule"))?;
    let colon = line.find(':')?;
    Some(line[colon + 1..].trim().to_string())
}

fn extract_field(src: &str, field: &str) -> Option<String> {
    for line in src.lines() {
        let pattern = format!("**{}:**", field);
        if line.contains(&pattern) {
            let after = line.splitn(2, &pattern).nth(1)?;
            return Some(after.trim().to_string());
        }
    }
    None
}

fn extract_body(src: &str) -> String {
    let mut found_dash = false;
    let mut body_lines: Vec<&str> = Vec::new();
    for line in src.lines() {
        if !found_dash {
            if line.trim() == "---" { found_dash = true; }
            continue;
        }
        body_lines.push(line);
    }
    while body_lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
        body_lines.remove(0);
    }
    body_lines.join("\n").trim().to_string()
}

fn escape_json_string(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => {},
            '\t' => out.push_str("\\t"),
            c    => out.push(c),
        }
    }
    out
}

pub(crate) fn generate_rules_json(rules: &[RuleInfo], out_path: &Path) {
    let mut out = String::new();
    out.push('[');
    for (i, r) in rules.iter().enumerate() {
        if i > 0 { out.push(','); }
        let scopes_json = r.scopes.iter()
            .map(|s| format!("\"{}\"", escape_json_string(s)))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "{{\"number\":{},\"slug\":\"{}\",\"title\":\"{}\",\"status\":\"{}\",\"severity\":\"{}\",\"scopes\":[{}],\"version\":\"{}\",\"date\":\"{}\",\"body\":\"{}\"}}",
            r.number,
            escape_json_string(&r.slug),
            escape_json_string(&r.title),
            escape_json_string(&r.status),
            escape_json_string(&r.severity),
            scopes_json,
            escape_json_string(&r.version),
            escape_json_string(&r.date),
            escape_json_string(&r.body),
        ));
    }
    out.push(']');
    fs::write(out_path, &out).unwrap();
    println!("cargo:warning=CanonRS Rules: rules.json ({} rules)", rules.len());
}

pub(crate) fn generate_rules_seo(rules: &[RuleInfo], out_path: &Path) {
    let mut out = String::new();
    out.push('[');
    for (i, r) in rules.iter().enumerate() {
        if i > 0 { out.push(','); }
        let description = first_paragraph(&r.body);
        let keywords = extract_keywords(r);
        let keywords_json = keywords.iter()
            .map(|k| format!("\"{}\"", escape_json_string(k)))
            .collect::<Vec<_>>()
            .join(",");
        let canonical = format!("https://canonrs.dev/rules/{}", r.slug);
        let scopes_json = r.scopes.iter()
            .map(|s| format!("\"{}\"", escape_json_string(s)))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!(
            "{{\"number\":{},\"slug\":\"{}\",\"title\":\"{}\",\"description\":\"{}\",\"keywords\":[{}],\"canonical\":\"{}\",\"severity\":\"{}\",\"scopes\":[{}]}}",
            r.number,
            escape_json_string(&r.slug),
            escape_json_string(&r.title),
            escape_json_string(&description),
            keywords_json,
            escape_json_string(&canonical),
            escape_json_string(&r.severity),
            scopes_json,
        ));
    }
    out.push(']');
    fs::write(out_path, &out).unwrap();
    println!("cargo:warning=CanonRS Rules: rules_seo.json ({} rules)", rules.len());
}

fn first_paragraph(body: &str) -> String {
    for line in body.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with("---") { continue; }
        if t.len() > 20 { return t.chars().take(200).collect(); }
    }
    String::new()
}

fn extract_keywords(r: &RuleInfo) -> Vec<String> {
    let mut kw: Vec<String> = r.scopes.clone();
    for line in r.body.lines() {
        let t = line.trim();
        if t.starts_with("## ") || t.starts_with("### ") {
            let heading = t.trim_start_matches('#').trim().to_lowercase();
            if heading.len() > 2 && heading.len() < 40 { kw.push(heading); }
        }
    }
    let mut rest = r.body.as_str();
    while let Some(start) = rest.find("**") {
        rest = &rest[start + 2..];
        if let Some(end) = rest.find("**") {
            let term = rest[..end].trim().to_lowercase();
            if term.len() > 2 && term.len() < 30 && !term.contains('\n') { kw.push(term); }
            rest = &rest[end + 2..];
        } else { break; }
    }
    kw.sort();
    kw.dedup();
    kw.truncate(10);
    kw
}

pub(crate) fn generate_rules_llm(rules: &[RuleInfo], out_path: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Canon Rules\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");
    for r in rules {
        out.push_str(&format!("## CR-{:03} — {}\n\n", r.number, r.title));
        out.push_str(&format!("- **Severity:** {}\n", r.severity));
        out.push_str(&format!("- **Status:** {}\n", r.status));
        out.push_str(&format!("- **Scope:** {}\n", r.scopes.join(", ")));
        out.push_str(&format!("- **Version:** {}\n\n", r.version));
        let sections = extract_sections(&r.body);
        if let Some(p) = sections.get("The Problem").or_else(|| sections.get("Problem")) {
            out.push_str(&format!("### Problem\n\n{}\n\n", p.chars().take(300).collect::<String>()));
        }
        if let Some(s) = sections.get("The Correct Solution").or_else(|| sections.get("Solution")).or_else(|| sections.get("The Solution")) {
            out.push_str(&format!("### Solution\n\n{}\n\n", s.chars().take(300).collect::<String>()));
        }
        let antipatterns: Vec<&str> = r.body.lines()
            .filter(|l| l.contains('❌'))
            .take(3)
            .collect();
        if !antipatterns.is_empty() {
            out.push_str("### Anti-patterns\n\n");
            for ap in antipatterns {
                out.push_str(&format!("- {}\n", ap.trim().trim_start_matches('❌').trim()));
            }
            out.push('\n');
        }
        out.push_str("---\n\n");
    }
    fs::write(out_path, &out).unwrap();
    println!("cargo:warning=CanonRS Rules: rules_llm.md ({} rules)", rules.len());
}

fn extract_sections(body: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let mut current_heading = String::new();
    let mut current_lines: Vec<&str> = Vec::new();
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("## ") {
            if !current_heading.is_empty() {
                map.insert(current_heading.clone(), current_lines.join(" ").trim().to_string());
            }
            current_heading = t.trim_start_matches('#').trim().to_string();
            current_lines.clear();
        } else if !current_heading.is_empty() && !t.starts_with('#') {
            current_lines.push(t);
        }
    }
    if !current_heading.is_empty() {
        map.insert(current_heading, current_lines.join(" ").trim().to_string());
    }
    map
}
