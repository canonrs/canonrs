use std::fs;
use std::path::Path;
use canonrs_tokens::design::tokens::semantics::{SEMANTIC_BRIDGES, SemanticBridge};

pub fn generate(output_dir: &Path) {
    let mut css = String::from("/* SEMANTIC LAYER — Bridge to Theme */\n:root {\n");

    for bridge in SEMANTIC_BRIDGES {
        css.push_str(&format!("  --{}: {};\n", bridge.token, bridge.theme_ref));
    }

    css.push_str("}\n");

    fs::write(output_dir.join("semantic.css"), css).ok();
    println!("  ✓ semantic.css (complete)");
}
