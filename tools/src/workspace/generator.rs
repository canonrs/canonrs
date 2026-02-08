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
    
    // Create workspace Cargo.toml with correct relative paths
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
bin-features = ["ssr"]
lib-features = ["hydrate"]
lib-profile-release = "wasm-release"
"#, profiles::generate_profiles_toml(mode), app_name, app_name, app_name);
    
    fs::write(workspace_dir.join("Cargo.toml"), workspace_cargo)?;
    
    // Copy app files into workspace/app/ directory
    let workspace_app_dir = workspace_dir.join("app");
    fs::create_dir_all(&workspace_app_dir)?;
    
    // Copy and FIX Cargo.toml (remove canonrs dependency for now)
    let cargo_content = fs::read_to_string(app_dir.join("Cargo.toml"))?;
    let fixed_cargo = cargo_content.replace(
        r#"canonrs = { path = "../../packages-rust/rs-canonrs/canonrs" }"#,
        "# canonrs = \"0.1\"  # TODO: Add canonrs dependency"
    );
    fs::write(workspace_app_dir.join("Cargo.toml"), fixed_cargo)?;
    
    // Copy src/
    copy_dir_recursive(&app_dir.join("src"), &workspace_app_dir.join("src"))?;
    
    // Create .gitignore
    fs::write(app_dir.join(".canonrs/.gitignore"), "*\n")?;
    fs::write(workspace_dir.join(".gitignore"), "target/\napp/\n")?;
    
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
