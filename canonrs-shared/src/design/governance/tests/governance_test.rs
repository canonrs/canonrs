#[cfg(test)]
mod governance {
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    fn no_duplicate_token_definitions() {
        let mut tokens: HashMap<String, Vec<String>> = HashMap::new();
        
        // Scanear todos os CSS
        scan_dir(Path::new("styles"), &mut tokens);
        
        let mut duplicates = Vec::new();
        for (token, locations) in tokens {
            if locations.len() > 1 {
                // Permitir apenas em themes/generated (múltiplos temas)
                let non_generated: Vec<_> = locations.iter()
                    .filter(|l| !l.contains("themes/generated/"))
                    .cloned()
                    .collect();
                
                if non_generated.len() > 1 || 
                   (non_generated.len() == 1 && locations.len() > 1) {
                    duplicates.push((token, locations));
                }
            }
        }
        
        assert!(
            duplicates.is_empty(),
            "Duplicate token definitions found:\n{:#?}",
            duplicates
        );
    }
    
    fn scan_dir(dir: &Path, tokens: &mut HashMap<String, Vec<String>>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && !path.to_str().unwrap().contains("node_modules") {
                    scan_dir(&path, tokens);
                } else if path.extension().map(|e| e == "css").unwrap_or(false) {
                    scan_file(&path, tokens);
                }
            }
        }
    }
    
    fn scan_file(path: &Path, tokens: &mut HashMap<String, Vec<String>>) {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if let Some(token) = line.trim().strip_prefix("--") {
                    if let Some(name) = token.split(':').next() {
                        tokens.entry(name.to_string())
                            .or_default()
                            .push(path.display().to_string());
                    }
                }
            }
        }
    }
}
