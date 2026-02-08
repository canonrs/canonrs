use std::path::Path;
use std::fs;
use anyhow::Result;

pub fn workspace_needs_regen(app_dir: &Path) -> Result<bool> {
    let workspace_dir = app_dir.join(".canonrs/workspace");
    let workspace_cargo = workspace_dir.join("Cargo.toml");

    if !workspace_cargo.exists() {
        return Ok(true);
    }

    let workspace_app_dir = workspace_dir.join("app");

    if !files_match(&app_dir.join("Cargo.toml"), &workspace_app_dir.join("Cargo.toml"))? {
        return Ok(true);
    }

    let canonrs_toml = app_dir.join("canonrs.toml");
    if canonrs_toml.exists() {
        let workspace_mtime = fs::metadata(&workspace_cargo)?.modified()?;
        let canonrs_mtime = fs::metadata(&canonrs_toml)?.modified()?;
        if canonrs_mtime > workspace_mtime {
            return Ok(true);
        }
    }

    Ok(false)
}

fn files_match(a: &Path, b: &Path) -> Result<bool> {
    if !a.exists() || !b.exists() {
        return Ok(false);
    }

    let content_a = fs::read_to_string(a)?;
    let content_b = fs::read_to_string(b)?;

    Ok(content_a == content_b)
}
