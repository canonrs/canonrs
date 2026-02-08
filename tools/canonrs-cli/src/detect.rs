use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Ssr,
    Csr,
    Hybrid,
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::Ssr => "ssr",
            Mode::Csr => "csr",
            Mode::Hybrid => "hybrid",
        }
    }

    pub fn profile_name(&self) -> &str {
        match self {
            Mode::Ssr => "canonrs-ssr",
            Mode::Csr => "canonrs-csr",
            Mode::Hybrid => "canonrs-hybrid",
        }
    }
}

pub fn detect_mode(app_dir: &Path) -> Result<Mode> {
    let config_path = app_dir.join("canonrs.toml");
    if !config_path.exists() {
        return Ok(Mode::Ssr); // Default
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: toml::Value = toml::from_str(&content)?;

    let mode_str = config
        .get("app")
        .and_then(|app| app.get("mode"))
        .and_then(|m| m.as_str())
        .unwrap_or("ssr");

    Ok(match mode_str {
        "csr" => Mode::Csr,
        "hybrid" => Mode::Hybrid,
        _ => Mode::Ssr,
    })
}

impl Mode {
    pub fn leptos_features(&self) -> (&'static str, &'static str) {
        match self {
            Mode::Ssr => ("ssr", "hydrate"),
            Mode::Csr => ("csr", "csr"),
            Mode::Hybrid => ("ssr", "hydrate"),
        }
    }
}
