use std::path::Path;

fn main() {
    let file_dir = Path::new("/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-server/styles");
    let import_path = "./.generated/primitives.css";
    let clean = import_path.strip_prefix("./").unwrap_or(import_path);
    let resolved = file_dir.join(clean);
    println!("Resolved: {}", resolved.display());
    println!("Exists: {}", resolved.exists());
}
