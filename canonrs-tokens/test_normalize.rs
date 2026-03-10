use std::path::{Path, PathBuf};

fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => { result.pop(); }
            c => result.push(c),
        }
    }
    result
}

fn main() {
    let path = Path::new("/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-server/styles/.generated/primitives.css");
    let norm = normalize(path);
    println!("Original: {}", path.display());
    println!("Normalized: {}", norm.display());
    println!("Exists: {}", norm.exists());
}
