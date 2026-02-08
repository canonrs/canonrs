use std::process::Command;
use anyhow::Result;
use colored::*;

pub fn execute() -> Result<()> {
    println!("{}", "🩺 CanonRS Health Check".cyan().bold());
    println!();

    print!("  Rust version... ");
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{} {}", "✓".green(), version.trim());
        }
        Err(_) => {
            println!("{} Rust not found", "✗".red());
        }
    }

    print!("  cargo-leptos... ");
    match Command::new("cargo").arg("leptos").arg("--version").output() {
        Ok(_) => println!("{} installed", "✓".green()),
        Err(_) => {
            println!("{} NOT installed", "✗".red());
            println!("     Install: cargo install cargo-leptos");
        }
    }

    print!("  wasm32 target... ");
    match Command::new("rustup").args(&["target", "list", "--installed"]).output() {
        Ok(output) => {
            let targets = String::from_utf8_lossy(&output.stdout);
            if targets.contains("wasm32-unknown-unknown") {
                println!("{} installed", "✓".green());
            } else {
                println!("{} NOT installed", "✗".red());
                println!("     Install: rustup target add wasm32-unknown-unknown");
            }
        }
        Err(_) => println!("{} cannot check", "?".yellow()),
    }

    print!("  canonrs.toml... ");
    if std::path::Path::new("canonrs.toml").exists() {
        println!("{} found", "✓".green());
    } else {
        println!("{} not found (are you in a CanonRS app?)", "✗".yellow());
    }

    print!("  .canonrs workspace... ");
    if std::path::Path::new(".canonrs/workspace").exists() {
        println!("{} initialized", "✓".green());
    } else {
        println!("{} not initialized (run 'canonrs dev')", "?".yellow());
    }

    println!();
    println!("{}", "Health check complete!".green().bold());

    Ok(())
}
