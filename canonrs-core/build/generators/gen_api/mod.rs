//! gen_api — entry points públicos

mod parser;
mod resolver;
mod renderer;

use std::fs;
use std::path::Path;
use parser::parse_components;
use renderer::render_api;

fn write_if_changed(path: &Path, new_content: &str) {
    if let Ok(existing) = fs::read_to_string(path) {
        if existing == new_content {
            return; // conteúdo idêntico — não escreve, watcher não detecta mudança
        }
    }
    fs::write(path, new_content).unwrap();
}

pub(crate) fn generate_api_files(ui_dir: &Path) {
    let entries = match fs::read_dir(ui_dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut count = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let ui_file  = path.join(format!("{}_ui.rs", dir_name));
        let content  = match fs::read_to_string(&ui_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let components = parse_components(&content);
        if components.is_empty() { continue; }
        write_if_changed(&path.join("api.rs"), &render_api(&dir_name, &components));
        count += 1;
    }
    println!("cargo:warning=CanonRS API: {} api.rs generated", count);
}

pub(crate) fn generate_api_files_blocks(blocks_dir: &Path) {
    generate_for_dir(blocks_dir, "block", "CanonRS API blocks");
}

pub(crate) fn generate_api_files_layouts(layouts_dir: &Path) {
    generate_for_dir(layouts_dir, "layout", "CanonRS API layouts");
}

fn generate_for_dir(dir: &Path, suffix: &str, label: &str) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut count = 0usize;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() { continue; }
        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let src_file = path.join(format!("{}_{}.rs", dir_name, suffix));
        let content  = match fs::read_to_string(&src_file) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let components = parse_components(&content);
        if components.is_empty() { continue; }
        write_if_changed(&path.join("api.rs"), &render_api(&dir_name, &components));
        count += 1;
    }
    println!("cargo:warning={}: {} api.rs generated", label, count);
}
