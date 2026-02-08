use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use colored::*;

pub struct Violation {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub message: String,
}

pub fn check_providers(base_path: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    
    let provider_path = base_path.join("src/providers");
    if !provider_path.exists() {
        return violations;
    }
    
    let allowed = ["theme_provider.rs", "theme_types.rs", "density_provider.rs", 
                   "density_types.rs", "language_provider.rs", "language_types.rs",
                   "theme_engine.rs", "mod.rs"];
    
    for entry in WalkDir::new(&provider_path).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if !path.is_file() { continue; }
        
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.ends_with(".rs") && !allowed.contains(&filename) {
            violations.push(Violation {
                file: path.display().to_string(),
                line: 1,
                rule: "Canon #37".to_string(),
                message: format!("Forbidden provider: {}", filename),
            });
        }
    }
    
    // Check for forbidden patterns
    let forbidden = ["PreferencesProvider", "SettingsProvider", "UISettingsProvider"];
    
    for entry in WalkDir::new(base_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
        
        let content = fs::read_to_string(path).unwrap_or_default();
        
        for pattern in &forbidden {
            if content.contains(pattern) {
                violations.push(Violation {
                    file: path.display().to_string(),
                    line: 0,
                    rule: "Canon #37".to_string(),
                    message: format!("Forbidden provider: {}", pattern),
                });
            }
        }
    }
    
    violations
}

pub fn check_no_localstorage(base_path: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    let provider_path = base_path.join("src/providers");
    
    if !provider_path.exists() {
        return violations;
    }
    
    for entry in WalkDir::new(&provider_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
        
        let content = fs::read_to_string(path).unwrap_or_default();
        
        for (line_num, line) in content.lines().enumerate() {
            if line.contains("localStorage") || line.contains("sessionStorage") {
                violations.push(Violation {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    rule: "Canon #32".to_string(),
                    message: "No localStorage in providers".to_string(),
                });
            }
        }
    }
    
    violations
}

pub fn check_single_theme_engine(base_path: &Path) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut count = 0;
    
    for entry in WalkDir::new(base_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
        
        let content = fs::read_to_string(path).unwrap_or_default();
        let regex = Regex::new(r"pub fn ThemeEngine\(").unwrap();
        
        if regex.is_match(&content) {
            count += 1;
        }
    }
    
    if count > 1 {
        violations.push(Violation {
            file: "multiple files".to_string(),
            line: 0,
            rule: "Canon #38".to_string(),
            message: format!("Multiple ThemeEngine found ({})", count),
        });
    }
    
    violations
}
