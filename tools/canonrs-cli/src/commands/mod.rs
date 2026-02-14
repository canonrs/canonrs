pub mod new;
pub mod dev;
pub mod build;
pub mod doctor;

pub fn canonrs_root() -> anyhow::Result<std::path::PathBuf> {
    use std::path::{Path, PathBuf};

    if let Ok(root) = std::env::var("CANONRS_ROOT") {
        let path = PathBuf::from(&root);
        if path.join("canonrs-tokens").exists() {
            return Ok(path);
        }
        anyhow::bail!(
            "CANONRS_ROOT is set but canonrs-tokens/ not found inside:\n{}",
            root
        );
    }

    let exe = std::env::current_exe()?;
    let mut dir = exe
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    for _ in 0..12 {
        if dir.join("canonrs-tokens").exists() {
            return Ok(dir);
        }
        if let Some(parent) = dir.parent() {
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    anyhow::bail!(
        "Could not locate canonrs-tokens/.\n\
         CanonRS CLI must run inside the rs-canonrs workspace.\n\n\
         Fix:\n\
           export CANONRS_ROOT=/abs/path/to/rs-canonrs\n"
    )
}
