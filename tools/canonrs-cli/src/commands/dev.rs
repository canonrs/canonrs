use std::process::Command;
use anyhow::{Result, Context};
use colored::*;
use crate::{detect, validate, workspace};

pub fn execute() -> Result<()> {
    let app_dir = std::env::current_dir()?;
    validate::check_no_profiles_in_app(&app_dir)?;

    println!("{}", "🎨 Running tokens-engine...".cyan());
    let tokens_dir = super::canonrs_root()?.join("canonrs-tokens");
    let tokens_status = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("tokens-engine")
        .current_dir(&tokens_dir)
        .status()
        .context("Failed to run tokens-engine")?;

    if !tokens_status.success() {
        anyhow::bail!("tokens-engine failed");
    }
    println!("{}", "  ✓ CSS generated".green());

    println!("{}", "🔧 Preparing workspace...".cyan());
    let mode = detect::detect_mode(&app_dir)?;
    println!("   Mode: {}", mode.as_str().yellow());

    let workspace_dir = app_dir.join(".canonrs/workspace");
    if workspace::cache::workspace_needs_regen(&app_dir)? {
        println!("   Regenerating workspace...");
        workspace::generator::generate_workspace(&app_dir, mode)?;
    } else {
        println!("   Workspace up to date");
    }

    println!("{}", "🚀 Starting dev server...".green());
    let status = Command::new("cargo")
        .current_dir(&workspace_dir)
        .arg("leptos")
        .arg("watch")
        .status()
        .context("Failed to run cargo leptos")?;

    if !status.success() {
        eprintln!();
        eprintln!("{}", "❌ Build failed in CanonRS pipeline".red().bold());
        eprintln!("{}", "This is NOT your fault.".yellow());
        eprintln!("{}", "Report at: https://github.com/canonrs/canonrs/issues".cyan());
        anyhow::bail!("Dev server exited with error");
    }

    Ok(())
}
