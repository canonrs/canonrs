use std::process::Command;
use std::{env, fs, path::PathBuf};

fn main() {
    // 1. EXECUTAR tokens-engine para garantir CSS fresh
    println!("cargo:warning=🔧 Running tokens-engine...");
    
    let status = Command::new("cargo")
        .args(["run", "--bin", "tokens-engine"])
        .current_dir("../canonrs-tokens")
        .status()
        .expect("Failed to run tokens-engine");
    
    if !status.success() {
        panic!("tokens-engine failed!");
    }
    
    // 2. COPIAR CSS gerado
    let css_source = PathBuf::from("../canonrs-ui/styles/canonrs.bundle.css");
    
    if !css_source.exists() {
        panic!("CSS not found after tokens-engine ran!");
    }
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir).join("canonrs.css");
    
    fs::copy(&css_source, &out_path).expect("Failed to copy CSS");
    
    let size = fs::metadata(&out_path).unwrap().len();
    println!("cargo:warning=✅ CanonRS CSS embedded: {} bytes", size);
    
    // 3. Rerun se tokens mudam
    println!("cargo:rerun-if-changed=../canonrs-tokens/src");
    println!("cargo:rerun-if-changed=../canonrs-ui/styles");
}
