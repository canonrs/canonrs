//! gen_rules.rs — Gera rules.json a partir dos .md
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub(crate) struct RuleInfo {
    pub number:        u32,
    pub slug:          String,
    pub title:         String,
    pub status:        String,
    pub severity:      String,
    pub category:      String,
    pub tags:          Vec<String>,
    pub language:      String,
    pub version:       String,
    pub date:          String,
    pub intro:         String,
    pub problem:       String,
    pub solution:      String,
    pub signals:       Vec<String>,
    pub search_intent: String,
    pub keywords:      Vec<String>,
    pub body:          String,
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

        let number   = extract_number_from_title(&src).or_else(|| extract_number_from_filename(&fname)).unwrap_or(0);
        let title    = extract_title(&src).unwrap_or_else(|| fname.clone());
        let status   = extract_field(&src, "Status").unwrap_or_else(|| "DRAFT".to_string());
        let severity = extract_field(&src, "Severity").unwrap_or_else(|| "LOW".to_string());
        let category = extract_field(&src, "Category").unwrap_or_default();
        let tags     = extract_field(&src, "Tags")
            .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
            .unwrap_or_default();
        let language = extract_field(&src, "Language").unwrap_or_else(|| "EN".to_string());
        let version  = extract_field(&src, "Version").unwrap_or_else(|| "1.0.0".to_string());
        let date     = extract_field(&src, "Date").unwrap_or_else(|| "2025-01-01".to_string());
        let intro    = extract_block(&src, "Intro").unwrap_or_default();
        let problem  = extract_block(&src, "Problem").unwrap_or_default();
        let solution = extract_block(&src, "Solution").unwrap_or_default();
        let signals  = extract_block(&src, "Signals")
            .map(|s| s.lines().filter(|l| l.starts_with("- ")).map(|l| l.trim_start_matches("- ").to_string()).collect())
            .unwrap_or_default();
        let search_intent = extract_block(&src, "Search Intent").unwrap_or_default();
        let keywords = extract_block(&src, "Keywords")
            .map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect())
            .unwrap_or_default();
        let body     = extract_body(&src);
        let slug     = fname.trim_start_matches("canon-rule-").to_string();

        rules.push(RuleInfo {
            number, slug, title, status, severity, category, tags, language,
            version, date, intro, problem, solution, signals, search_intent, keywords, body,
        });
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

fn extract_block(src: &str, field: &str) -> Option<String> {
    let pattern = format!("**{}:**", field);
    let mut lines = src.lines();
    while let Some(line) = lines.next() {
        if line.contains(&pattern) {
            // Coletar linhas até próximo campo ** ou ---
            let mut block = Vec::new();
            for next in lines.by_ref() {
                let t = next.trim();
                if t.starts_with("**") && t.contains(":**") { break; }
                if t == "---" { break; }
                block.push(next);
            }
            let result = block.join("\n").trim().to_string();
            if result.is_empty() { return None; }
            return Some(result);
        }
    }
    None
}

fn extract_body(src: &str) -> String {
    // Body começa após o segundo ---
    let mut dash_count = 0;
    let mut body_lines: Vec<&str> = Vec::new();
    let mut in_body = false;
    for line in src.lines() {
        if line.trim() == "---" {
            dash_count += 1;
            if dash_count == 2 { in_body = true; continue; }
        }
        if in_body { body_lines.push(line); }
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
        let tags_json     = r.tags.iter().map(|s| format!("\"{}\"", escape_json_string(s))).collect::<Vec<_>>().join(",");
        let signals_json  = r.signals.iter().map(|s| format!("\"{}\"", escape_json_string(s))).collect::<Vec<_>>().join(",");
        let keywords_json = r.keywords.iter().map(|s| format!("\"{}\"", escape_json_string(s))).collect::<Vec<_>>().join(",");
        out.push_str(&format!(
            "{{\"number\":{},\"slug\":\"{}\",\"title\":\"{}\",\"status\":\"{}\",\"severity\":\"{}\",\"category\":\"{}\",\"tags\":[{}],\"language\":\"{}\",\"version\":\"{}\",\"date\":\"{}\",\"intro\":\"{}\",\"problem\":\"{}\",\"solution\":\"{}\",\"signals\":[{}],\"search_intent\":\"{}\",\"keywords\":[{}],\"body\":\"{}\"}}",
            r.number,
            escape_json_string(&r.slug),
            escape_json_string(&r.title),
            escape_json_string(&r.status),
            escape_json_string(&r.severity),
            escape_json_string(&r.category),
            tags_json,
            escape_json_string(&r.language),
            escape_json_string(&r.version),
            escape_json_string(&r.date),
            escape_json_string(&r.intro),
            escape_json_string(&r.problem),
            escape_json_string(&r.solution),
            signals_json,
            escape_json_string(&r.search_intent),
            keywords_json,
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
        let keywords_json = r.keywords.iter().map(|s| format!("\"{}\"", escape_json_string(s))).collect::<Vec<_>>().join(",");
        let canonical = format!("https://canonrs.dev/rules/{}", r.slug);
        out.push_str(&format!(
            "{{\"number\":{},\"slug\":\"{}\",\"title\":\"{}\",\"description\":\"{}\",\"keywords\":[{}],\"canonical\":\"{}\",\"category\":\"{}\"}}",
            r.number,
            escape_json_string(&r.slug),
            escape_json_string(&r.title),
            escape_json_string(&r.intro),
            keywords_json,
            escape_json_string(&canonical),
            escape_json_string(&r.category),
        ));
    }
    out.push(']');
    fs::write(out_path, &out).unwrap();
    println!("cargo:warning=CanonRS Rules: rules_seo.json ({} rules)", rules.len());
}

pub(crate) fn generate_rules_llm(rules: &[RuleInfo], out_path: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Canon Rules\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");
    for r in rules {
        out.push_str(&format!("## CR-{:03} — {}\n\n", r.number, r.title));
        out.push_str(&format!("- **Category:** {}\n", r.category));
        out.push_str(&format!("- **Severity:** {}\n", r.severity));
        out.push_str(&format!("- **Status:** {}\n\n", r.status));
        if !r.problem.is_empty() {
            out.push_str(&format!("### Problem\n\n{}\n\n", r.problem));
        }
        if !r.solution.is_empty() {
            out.push_str(&format!("### Solution\n\n{}\n\n", r.solution));
        }
        if !r.signals.is_empty() {
            out.push_str("### Signals\n\n");
            for s in &r.signals { out.push_str(&format!("- {}\n", s)); }
            out.push('\n');
        }
        out.push_str("---\n\n");
    }
    fs::write(out_path, &out).unwrap();
    println!("cargo:warning=CanonRS Rules: rules_llm.md ({} rules)", rules.len());
}
