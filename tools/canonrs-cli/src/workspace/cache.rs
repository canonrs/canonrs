use std::path::Path;
use std::fs;
use anyhow::Result;

pub fn workspace_needs_regen(app_dir: &Path) -> Result<bool> {
    let workspace_cargo = app_dir.join(".canonrs/workspace/Cargo.toml");

    // 1. Workspace não existe → regen
    if !workspace_cargo.exists() {
        return Ok(true);
    }

    let workspace_mtime = fs::metadata(&workspace_cargo)?.modified()?;

    // 2. canonrs.toml mudou → regen
    let canonrs_toml = app_dir.join("canonrs.toml");
    if canonrs_toml.exists() {
        let canonrs_mtime = fs::metadata(&canonrs_toml)?.modified()?;
        if canonrs_mtime > workspace_mtime {
            return Ok(true);
        }
    }

    // 3. app Cargo.toml mudou → regen
    let app_cargo = app_dir.join("Cargo.toml");
    if app_cargo.exists() {
        let app_mtime = fs::metadata(&app_cargo)?.modified()?;
        if app_mtime > workspace_mtime {
            return Ok(true);
        }
    }

    Ok(false)
}
