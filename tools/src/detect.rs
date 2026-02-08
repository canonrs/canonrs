use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    SSR,
    CSR,
    Hybrid,
}

pub fn detect_mode(app_dir: &Path) -> Result<Mode> {
    let cargo_toml = app_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(cargo_toml)?;
    
    if content.contains("leptos_axum") || content.contains("leptos-axum") {
        Ok(Mode::SSR)
    } else if content.contains("leptos") {
        Ok(Mode::CSR)
    } else {
        Ok(Mode::Hybrid)
    }
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::SSR => "ssr",
            Mode::CSR => "csr",
            Mode::Hybrid => "hybrid",
        }
    }
    
    pub fn profile_name(&self) -> &str {
        match self {
            Mode::SSR => "canonrs-ssr",
            Mode::CSR => "canonrs-csr",
            Mode::Hybrid => "canonrs-hybrid",
        }
    }
}
