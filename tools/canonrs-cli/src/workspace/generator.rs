use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use crate::detect::Mode;
use super::profiles;

pub fn generate_workspace(app_dir: &Path, mode: Mode) -> Result<()> {
    let workspace_dir = app_dir.join(".canonrs/workspace");
    fs::create_dir_all(&workspace_dir)?;

    let app_name = app_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("app");

    let profile_name = mode.profile_name();
    let (bin_features, lib_features) = mode.leptos_features();
    let app_real = app_dir.canonicalize()
        .context("Failed to canonicalize app_dir")?;

    let workspace_cargo = format!(r#"[workspace]
members = ["{app_path}"]
resolver = "2"

{profiles}

[[workspace.metadata.leptos]]
name = "{name}"
bin-package = "{name}"
lib-package = "{name}"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "{app_path}/style/main.css"
assets-dir = "{app_path}/public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
bin-features = ["{bin_feat}"]
lib-features = ["{lib_feat}"]
lib-profile-release = "wasm-release"
bin-profile-release = "{profile}"
"#,
        app_path = app_real.display(),
        profiles = profiles::generate_profiles_toml(mode),
        name = app_name,
        bin_feat = bin_features,
        lib_feat = lib_features,
        profile = profile_name,
    );

    fs::write(workspace_dir.join("Cargo.toml"), workspace_cargo)?;

    let gitignore_path = app_dir.join(".canonrs/.gitignore");
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, "*\n")?;
    }
    let workspace_gitignore = workspace_dir.join(".gitignore");
    if !workspace_gitignore.exists() {
        fs::write(&workspace_gitignore, "target/\n")?;
    }

    Ok(())
}
