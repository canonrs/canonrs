//! Extrai ComponentDef a partir do conteúdo de *_ui.rs

use super::resolver::resolve_type;

#[derive(Debug)]
pub(crate) struct PropDef {
    pub name:     String,
    pub ty:       super::resolver::PropKind,
    pub required: bool,
    pub default:  Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct ComponentDef {
    pub name:     String,
    pub props:    Vec<PropDef>,
    pub children: bool,
}

pub(crate) fn parse_components(content: &str) -> Vec<ComponentDef> {
    let mut components = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line == "#[component]" {
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() { j += 1; }
            if j >= lines.len() { i += 1; continue; }
            let fn_line = lines[j].trim();
            let is_pub = fn_line.starts_with("pub fn ") || fn_line.starts_with("pub(crate) fn ");
            if !is_pub { i += 1; continue; }
            let name = fn_line
                .trim_start_matches("pub fn ")
                .trim_start_matches("pub(crate) fn ")
                .split('(').next().unwrap_or("").trim().to_string();
            if name.ends_with("Preview") || name.ends_with("Example") { i = j + 1; continue; }
            let mut sig_lines = vec![fn_line.to_string()];
            let mut k = j + 1;
            while k < lines.len() {
                let l = lines[k].trim();
                sig_lines.push(l.to_string());
                if l.contains(") -> impl IntoView") || l.contains(") -> leptos") { break; }
                k += 1;
            }
            let sig      = sig_lines.join(" ");
            let children = sig.contains("children: Children");
            let props    = parse_props(&sig, content);
            components.push(ComponentDef { name, props, children });
            i = k + 1;
            continue;
        }
        i += 1;
    }
    components
}

fn parse_props(sig: &str, full_content: &str) -> Vec<PropDef> {
    let mut props = Vec::new();
    let start = match sig.find('(') { Some(s) => s + 1, None => return props };
    let end   = match sig.rfind(") ->") { Some(e) => e, None => return props };
    let args  = &sig[start..end];
    let params = split_params(args);
    let mut pending_attrs = String::new();

    for param in &params {
        let p = param.trim();
        if p.starts_with("#[") {
            if let Some(bracket_end) = p.find(']') {
                let attr_part = p[..=bracket_end].to_string();
                let rest = p[bracket_end + 1..].trim();
                if rest.contains(':') {
                    // attr + arg no mesmo token
                    process_arg(rest, &attr_part, full_content, &mut props);
                } else {
                    // attr sozinho — próximo token será o arg
                    pending_attrs = attr_part;
                }
            }
            continue;
        }
        if p.contains(':') && !p.starts_with("//") {
            process_arg(p, &pending_attrs, full_content, &mut props);
            pending_attrs.clear();
        }
    }
    props
}

fn process_arg(p: &str, attrs: &str, full_content: &str, props: &mut Vec<PropDef>) {
    let parts: Vec<&str> = p.splitn(2, ':').collect();
    if parts.len() != 2 { return; }
    let arg_name = parts[0].trim().to_string();
    let arg_type = parts[1].trim().trim_end_matches(',').to_string();
    if arg_name == "children" || arg_name.is_empty() { return; }
    let optional = attrs.contains("optional") || arg_type.starts_with("Option<");
    let raw_default = extract_default(attrs);
    let ty          = resolve_type(&arg_type, &arg_name, full_content);
    let default     = raw_default.map(|d| normalize_default(&d));
    props.push(PropDef {
        name: arg_name,
        ty,
        required: !optional && default.is_none(),
        default,
    });
}

fn extract_default(attrs: &str) -> Option<String> {
    if let Some(pos) = attrs.find("default = ") {
        let after = &attrs[pos + "default = ".len()..];
        // Captura até )] que fecha o #[prop(...)] ou até vírgula fora de parênteses
        let end = {
            let mut depth = 0usize;
            let mut found = after.len();
            let chars: Vec<char> = after.chars().collect();
            let mut ci = 0;
            while ci < chars.len() {
                match chars[ci] {
                    '(' => depth += 1,
                    ')' => {
                        if depth == 0 { found = ci; break; }
                        depth -= 1;
                    }
                    ',' if depth == 0 => { found = ci; break; }
                    _ => {}
                }
                ci += 1;
            }
            found
        };
        let val   = after[..end].trim().to_string();
        if !val.is_empty() { return Some(val); }
    }
    if attrs.contains("default") && !attrs.contains("default =") {
        return Some("Default::default()".to_string());
    }
    None
}

fn split_params(s: &str) -> Vec<String> {
    let mut result  = Vec::new();
    let mut depth   = 0usize;
    let mut current = String::new();
    let mut in_str  = false;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            current.push(c);
            if c == '"' { in_str = false; }
        } else {
            match c {
                '"' => { in_str = true; current.push(c); }
                '<' | '(' | '[' => { depth += 1; current.push(c); }
                '>' | ')' | ']' => { if depth > 0 { depth -= 1; } current.push(c); }
                ',' if depth == 0 => {
                    result.push(current.trim().to_string());
                    current = String::new();
                }
                _ => { current.push(c); }
            }
        }
        i += 1;
    }
    if !current.trim().is_empty() { result.push(current.trim().to_string()); }
    result
}

/// Normaliza defaults Rust para valores finais legíveis
/// Ex: "ButtonVariant::Primary" → "primary"
///     "String::new()" → ""
///     "\"text\".to_string()" → "text"
///     "true" → "true"
fn normalize_default(d: &str) -> String {
    let d = d.trim();
    if d == "String::new()" { return String::new(); }
    if d == "Default::default()" { return String::new(); }
    if d.starts_with("String::from(") && d.ends_with(')') {
        let inner = &d["String::from(".len()..d.len()-1];
        return inner.trim().trim_matches('"').to_string();
    }
    if d.ends_with(".to_string()") {
        let inner = d.trim_end_matches(".to_string()").trim();
        return inner.trim_matches('"').to_string();
    }
    if d.contains("::") && !d.contains('(') {
        let variant = d.split("::").last().unwrap_or(d);
        let mut result = String::new();
        for (i, c) in variant.chars().enumerate() {
            if c.is_uppercase() && i > 0 { result.push('-'); }
            result.push(c.to_lowercase().next().unwrap());
        }
        return result;
    }
    if d.contains("::") && d.contains('(') {
        let before_paren = d.split('(').next().unwrap_or(d);
        let variant = before_paren.split("::").last().unwrap_or(before_paren);
        let mut result = String::new();
        for (i, c) in variant.chars().enumerate() {
            if c.is_uppercase() && i > 0 { result.push('-'); }
            result.push(c.to_lowercase().next().unwrap());
        }
        return result;
    }
    d.to_string()
}
