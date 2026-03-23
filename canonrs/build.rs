use std::process::Command;
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:warning=🔧 Running tokens-engine...");

    let status = Command::new("cargo")
        .args(["run", "--bin", "tokens-engine"])
        .env("CARGO_INCREMENTAL", "0")
        .current_dir("../canonrs-tokens")
        .status()
        .expect("Failed to run tokens-engine");

    if !status.success() {
        panic!("tokens-engine failed!");
    }

    let css_source = PathBuf::from("../canonrs-server/styles/canonrs.bundle.css");

    if !css_source.exists() {
        panic!("CSS not found after tokens-engine ran!");
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir).join("canonrs.css");

    fs::copy(&css_source, &out_path).expect("Failed to copy CSS");

    let size = fs::metadata(&out_path).unwrap().len();
    println!("cargo:warning=✅ CanonRS CSS embedded: {} bytes", size);

    println!("cargo:rerun-if-changed=../canonrs-tokens/src");
    println!("cargo:rerun-if-changed=../canonrs-server/styles");
}
