use std::path::Path;
use anyhow::{Result, bail};
use colored::*;

pub fn check_no_profiles_in_app(app_dir: &Path) -> Result<()> {
    let cargo_toml = app_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)?;
    
    if content.contains("[profile.") {
        eprintln!("{}", "❌ ERROR: Profiles detected in Cargo.toml".red().bold());
        eprintln!();
        eprintln!("{}", "Profiles are managed by CanonRS and must NOT be in your Cargo.toml.".yellow());
        eprintln!("{}", "Remove all [profile.*] sections.".yellow());
        eprintln!();
        eprintln!("{}", "CanonRS automatically sets correct profiles based on canonrs.toml mode.".cyan());
        bail!("Invalid Cargo.toml");
    }
    
    Ok(())
}
