use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod workspace;
mod detect;
mod validate;

#[derive(Parser)]
#[command(name = "canonrs")]
#[command(about = "CanonRS Framework CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new CanonRS app
    New { name: String },
    /// Start development server
    Dev,
    /// Build for production
    Build,
    /// Check CanonRS installation and environment
    Doctor,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            commands::new::execute(&name)?;
        }
        Commands::Dev => {
            commands::dev::execute()?;
        }
        Commands::Build => {
            commands::build::execute()?;
        }
        Commands::Doctor => {
            commands::doctor::execute()?;
        }
    }

    Ok(())
}
