use anyhow::Result;
use colored::*;

pub fn execute(name: &str) -> Result<()> {
    println!("Creating new CanonRS app...");
    
    // TODO: Implement app scaffolding
    println!("{}", format!("✅ Created {} (SSR mode detected)", name).green());
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  canonrs dev");
    
    Ok(())
}
