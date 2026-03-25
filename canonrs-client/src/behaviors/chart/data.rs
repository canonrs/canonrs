//! Chart Data - parse e leitura de dados

#[cfg(feature = "hydrate")]
pub fn read_chart_data(root: &web_sys::Element) -> String {
    if let Ok(Some(script)) = root.query_selector("script[data-rs-chart-data]") {
        if let Some(text) = script.text_content() {
            return text;
        }
    }
    root.get_attribute("data-rs-chart-data").unwrap_or_default()
}

#[cfg(feature = "hydrate")]
pub fn parse_chart_data(json: &str) -> Option<(Vec<String>, Vec<(String, Vec<f64>, String, bool)>)> {
    let labels = extract_json_array(json, "labels")?;
    let series_json = extract_json_key(json, "series")?;
    let colors = ["#6366f1","#f59e0b","#10b981","#ef4444","#8b5cf6","#06b6d4"];
    let mut series = Vec::new();
    for (i, item) in split_json_objects(&series_json).iter().enumerate() {
        let name  = extract_json_string(item, "name").unwrap_or_else(|| format!("Series {}", i+1));
        let data  = extract_json_number_array(item, "data").unwrap_or_default();
        let color = extract_json_string(item, "color")
            .filter(|c| !c.is_empty())
            .unwrap_or_else(|| colors[i % colors.len()].to_string());
        series.push((name, data, color, true));
    }
    Some((labels, series))
}

#[cfg(feature = "hydrate")]
pub fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    if r.starts_with('"') { let e = r[1..].find('"')? + 1; Some(r[1..e].to_string()) } else { None }
}

#[cfg(feature = "hydrate")]
pub fn extract_json_key(json: &str, key: &str) -> Option<String> {
    let p = format!("\"{}\":", key);
    let s = json.find(&p)? + p.len();
    let r = json[s..].trim_start();
    let (open, close) = if r.starts_with('[') { ('[', ']') } else { ('{', '}') };
    let mut depth = 0; let mut end = 0;
    for (i, c) in r.chars().enumerate() {
        if c == open { depth += 1; }
        if c == close { depth -= 1; if depth == 0 { end = i + 1; break; } }
    }
    if end > 0 { Some(r[..end].to_string()) } else { None }
}

#[cfg(feature = "hydrate")]
pub fn extract_json_array(json: &str, key: &str) -> Option<Vec<String>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').map(|s| s.trim().trim_matches('"').to_string()).filter(|s| !s.is_empty()).collect())
}

#[cfg(feature = "hydrate")]
pub fn extract_json_number_array(json: &str, key: &str) -> Option<Vec<f64>> {
    let arr = extract_json_key(json, key)?;
    let inner = arr.trim_start_matches('[').trim_end_matches(']');
    Some(inner.split(',').filter_map(|s| s.trim().parse::<f64>().ok()).collect())
}

#[cfg(feature = "hydrate")]
pub fn split_json_objects(json: &str) -> Vec<String> {
    let inner = json.trim_start_matches('[').trim_end_matches(']');
    let mut out = Vec::new(); let mut depth = 0; let mut start = 0;
    for (i, c) in inner.chars().enumerate() {
        match c {
            '{' => { if depth == 0 { start = i; } depth += 1; }
            '}' => { depth -= 1; if depth == 0 { out.push(inner[start..=i].to_string()); } }
            _ => {}
        }
    }
    out
}
