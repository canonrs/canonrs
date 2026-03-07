use std::process::Command;
use anyhow::{Result, Context};
use colored::*;
use crate::{detect, validate};

pub fn execute() -> Result<()> {
    let app_dir = std::env::current_dir()?;
    validate::check_no_profiles_in_app(&app_dir)?;

    println!("{}", "🎨 Running tokens-engine...".cyan());
    let tokens_dir = super::canonrs_root()?.join("canonrs-tokens");
    let tokens_status = Command::new("cargo")
        .args(["run", "--release", "--bin", "tokens-engine"])
        .current_dir(&tokens_dir)
        .status()
        .context("Failed to run tokens-engine")?;

    if !tokens_status.success() {
        anyhow::bail!("tokens-engine failed");
    }
    println!("{}", "  ✓ CSS generated".green());

    println!("{}", "🔧 Building for production...".cyan());
    let mode = detect::detect_mode(&app_dir)?;
    println!("   Mode: {}", mode.as_str().yellow());

    let workspace_dir = app_dir.join(".canonrs/workspace");
    if !workspace_dir.exists() {
        crate::workspace::generator::generate_workspace(&app_dir, mode)?;
    }

    let status = Command::new("cargo")
        .current_dir(&workspace_dir)
        .args(["leptos", "build", "--release"])
        .status()
        .context("Failed to build")?;

    if !status.success() {
        anyhow::bail!("Build failed");
    }

    // Optimize WASM
    println!("{}", "⚡ Optimizing WASM...".cyan());
    let app_name = app_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("app")
        .replace('-', "_");

    let wasm_path = format!("target/site/pkg/{}.wasm", app_name);
    let wasm_full = app_dir.join(&wasm_path);

    if wasm_full.exists() {
        let opt_status = Command::new("wasm-opt")
            .args([
                "-Oz",
                "--strip-debug",
                "--strip-producers",
                wasm_full.to_str().unwrap(),
                "-o",
                wasm_full.to_str().unwrap(),
            ])
            .status()
            .context("Failed to run wasm-opt")?;

        if opt_status.success() {
            let size = std::fs::metadata(&wasm_full)?.len();
            println!("{}", format!("  ✓ WASM optimized: {:.1}MB", size as f64 / 1_048_576.0).green());
        }
    }

    println!("{}", "✅ Build complete".green());
    Ok(())
}
