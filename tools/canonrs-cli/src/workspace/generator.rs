use std::path::Path;
use std::fs;
use anyhow::Result;
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

    let workspace_cargo = format!(r#"
[workspace]
members = ["app"]
resolver = "2"

{}

[[workspace.metadata.leptos]]
name = "{}"
bin-package = "{}"
lib-package = "{}"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "../../style/main.css"
assets-dir = "../../public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
bin-features = ["{}"]
lib-features = ["{}"]
lib-profile-release = "wasm-release"
bin-profile-release = "{}"
"#, 
    profiles::generate_profiles_toml(mode), 
    app_name, 
    app_name, 
    app_name, 
    bin_features, 
    lib_features, 
    profile_name
);

    fs::write(workspace_dir.join("Cargo.toml"), workspace_cargo)?;

    let workspace_app_dir = workspace_dir.join("app");
    fs::create_dir_all(&workspace_app_dir)?;

    // Copy and adjust Cargo.toml paths
    let cargo_content = fs::read_to_string(app_dir.join("Cargo.toml"))?;
    
    // Adjust path: from app root "../../" to workspace/app "../../../../"
    let fixed_cargo = cargo_content.replace(
        r#"path = "../../packages-rust/rs-canonrs/canonrs""#,
        r#"path = "../../../../../packages-rust/rs-canonrs/canonrs""#
    );

    fs::write(workspace_app_dir.join("Cargo.toml"), fixed_cargo)?;

    copy_dir_recursive(&app_dir.join("src"), &workspace_app_dir.join("src"))?;

    // Create .gitignore only if doesn't exist
    let gitignore_path = app_dir.join(".canonrs/.gitignore");
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, "*\n")?;
    }

    let workspace_gitignore = workspace_dir.join(".gitignore");
    if !workspace_gitignore.exists() {
        fs::write(&workspace_gitignore, "target/\napp/\n")?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
