use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct TokenViolation {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub found: String,
}

pub fn check_tokens(base_path: &Path) -> Vec<TokenViolation> {
    let mut violations = Vec::new();
    
    for entry in WalkDir::new(base_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
        
        let content = fs::read_to_string(path).unwrap_or_default();
        
        // Skip if annotated
        if content.lines().take(15).any(|l| l.contains("@canon-level: loose")) {
            continue;
        }
        
        violations.extend(check_colors(&path, &content));
        violations.extend(check_sizes(&path, &content));
    }
    
    violations
}

fn check_colors(path: &Path, content: &str) -> Vec<TokenViolation> {
    let mut violations = Vec::new();
    let regex = Regex::new(r"(bg|text|border|ring)-(slate|gray|zinc|red|blue|green|yellow|purple|pink)-(\d+)").unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if line.contains("@canon-exempt") { continue; }
        
        if let Some(cap) = regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                rule: "Canon #21".to_string(),
                found: cap.get(0).unwrap().as_str().to_string(),
            });
        }
    }
    
    violations
}

fn check_sizes(path: &Path, content: &str) -> Vec<TokenViolation> {
    let mut violations = Vec::new();
    let regex = Regex::new(r"(h|w|p|m|gap)-\[(\d+)px\]").unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if line.contains("@canon-exempt") { continue; }
        
        if let Some(cap) = regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                rule: "Canon #24/#33".to_string(),
                found: cap.get(0).unwrap().as_str().to_string(),
            });
        }
    }
    
    violations
}
