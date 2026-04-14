use std::fs;
use std::path::Path;
use canonrs_tokens::design::tokens::semantics::{SEMANTICS_SURFACE, SEMANTICS_ACTIONS, SEMANTICS_STATES};

pub fn generate(output_dir: &Path) {
    let mut css = String::from("/* SEMANTIC LAYER — Bridge to Theme */\n[data-theme] {\n");

    for token in SEMANTICS_SURFACE {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }
    for token in SEMANTICS_ACTIONS {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }
    for token in SEMANTICS_STATES {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }

    css.push_str("}\n");

    fs::write(output_dir.join("semantic.css"), css).ok();
    println!("  ✓ semantic.css (scoped to [data-theme])");
}
