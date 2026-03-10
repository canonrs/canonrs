use std::fs;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

pub fn generate(styles_dir: &Path, output_dir: &Path) -> std::io::Result<()> {
    // Canonicalize para resolver paths relativos corretamente
    let styles_abs = styles_dir.canonicalize()?;
    let entry = styles_abs.join("canonrs.css");
    let output = output_dir.join("canonrs.bundle.css");

    let mut bundled = String::new();
    process_file(&entry, &styles_abs, &mut bundled)?;

    fs::write(&output, &bundled)?;
    let lines = bundled.lines().count();
    println!("  ✓ canonrs.bundle.css ({} lines)", lines);
    Ok(())
}

fn process_file(file: &Path, base_dir: &Path, output: &mut String) -> io::Result<()> {
    let reader = BufReader::new(fs::File::open(file)?);
    let file_dir = file.parent().unwrap_or(base_dir);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("@layer") {
            continue;
        }

        if trimmed.starts_with("@import") {
            if let Some(import_path) = extract_import_path(trimmed) {
                let clean_path = import_path.strip_prefix("./").unwrap_or(import_path);
                let resolved = file_dir.join(clean_path);

                if resolved.exists() {
                    output.push_str(&format!("/* Bundled: {} */\n", import_path));
                    process_file(&resolved, base_dir, output)?;
                } else {
                    output.push_str(&format!("/* Missing: {} */\n", import_path));
                }
            }
        } else {
            output.push_str(&line);
            output.push('\n');
        }
    }

    Ok(())
}

fn extract_import_path(line: &str) -> Option<&str> {
    let line = line.trim_start_matches("@import").trim();

    let without_quotes = if line.starts_with('"') && line.contains('"') {
        line.trim_start_matches('"').split('"').next()?
    } else if line.starts_with('\'') && line.contains('\'') {
        line.trim_start_matches('\'').split('\'').next()?
    } else {
        return None;
    };

    Some(without_quotes)
}
