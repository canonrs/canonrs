use std::process::Command;
use anyhow::{Result, Context};
use colored::*;
use crate::{detect, validate, workspace};

pub fn execute() -> Result<()> {
    let app_dir = std::env::current_dir()?;

    // 🔴 BLOQUEIO: verificar profiles proibidos
    validate::check_no_profiles_in_app(&app_dir)?;

    println!("{}", "🔧 Preparing workspace...".cyan());

    // Detect mode
    let mode = detect::detect_mode(&app_dir)?;
    println!("   Mode: {}", mode.as_str().yellow());

    // 🟠 CACHE: regenerar apenas se necessário
    let workspace_dir = app_dir.join(".canonrs/workspace");
    if workspace::cache::workspace_needs_regen(&app_dir)? {
        println!("   Regenerating workspace...");
        workspace::generator::generate_workspace(&app_dir, mode)?;
    } else {
        println!("   Workspace up to date");
    }

    println!("{}", "🚀 Starting dev server...".green());

    // Run cargo leptos watch with profile via env var
    let status = Command::new("cargo")
        .current_dir(&workspace_dir)
        .env("CARGO_PROFILE", mode.profile_name())
        .arg("leptos")
        .arg("watch")
        .status()
        .context("Failed to run cargo leptos")?;

    if !status.success() {
        eprintln!();
        eprintln!("{}", "❌ Build failed in CanonRS pipeline".red().bold());
        eprintln!();
        eprintln!("{}", "This is NOT your fault.".yellow());
        eprintln!("{}", "This is a known limitation of the Rust + LLVM toolchain in SSR contexts.".yellow());
        eprintln!();
        eprintln!("{}", "Report at:".cyan());
        eprintln!("{}", "https://github.com/canonrs/canonrs/issues".cyan().underline());
        eprintln!();
        anyhow::bail!("Dev server exited with error");
    }

    Ok(())
}
