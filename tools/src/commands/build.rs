use std::process::Command;
use anyhow::{Result, Context};
use colored::*;
use crate::{detect, validate};

pub fn execute() -> Result<()> {
    let app_dir = std::env::current_dir()?;
    validate::check_no_profiles_in_app(&app_dir)?;

    println!("{}", "🔧 Building for production...".cyan());

    let mode = detect::detect_mode(&app_dir)?;
    println!("   Mode: {}", mode.as_str().yellow());

    let workspace_dir = app_dir.join(".canonrs/workspace");
    if !workspace_dir.exists() {
        crate::workspace::generator::generate_workspace(&app_dir, mode)?;
    }

    let status = Command::new("cargo")
        .current_dir(&workspace_dir)
        .arg("leptos")
        .arg("build")
        .arg("--profile")
        .arg(mode.profile_name())
        .arg("--release")
        .status()
        .context("Failed to build")?;

    if !status.success() {
        anyhow::bail!("Build failed");
    }

    println!("{}", "✅ Build complete".green());
    Ok(())
}
