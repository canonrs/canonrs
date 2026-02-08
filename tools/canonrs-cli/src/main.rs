use clap::{Parser, Subcommand};

mod commands;
mod detect;
mod validate;
mod workspace;

#[derive(Parser)]
#[command(name = "canonrs")]
#[command(about = "CanonRS Framework CLI", version)]
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
    /// Health check
    Doctor,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => commands::new::execute(&name),
        Commands::Dev => commands::dev::execute(),
        Commands::Build => commands::build::execute(),
        Commands::Doctor => commands::doctor::execute(),
    }
}
