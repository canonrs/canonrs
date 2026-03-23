mod theme_generator;
mod entry_generator;
mod bundler;
mod semantic_generator;
mod root_generator;

use canonrs_tokens::design::tokens::families::*;
use canonrs_tokens::design::tokens::primitives::PRIMITIVE_VALUES;
use canonrs_tokens::design::tokens::core::CORE_TOKENS;
use std::fs;
use std::path::Path;

fn main() {
    let generated_path = Path::new("../canonrs-server/styles/.generated");
    let styles_path = Path::new("../canonrs-server/styles");
    let bundle_output_path = Path::new("../canonrs-server/styles");

    fs::create_dir_all(generated_path).expect("Failed to create dir");

    println!("🔧 Step 1: Generating primitives...");
    generate_primitives(generated_path);

    println!("\n🔧 Step 2: Generating foundation (core)...");
    generate_core(generated_path);

    println!("\n🔧 Step 3: Generating families...");
    generate_family("family-a-overlay", FAMILY_A_OVERLAY, generated_path);
    generate_family("family-b-selection", FAMILY_B_SELECTION, generated_path);
    generate_family("family-c-forms", FAMILY_C_FORMS, generated_path);
    generate_family("family-d-navigation", FAMILY_D_NAVIGATION, generated_path);
    generate_family("family-e-feedback", FAMILY_E_FEEDBACK, generated_path);
    generate_family("family-f-data", FAMILY_F_DATA, generated_path);
    generate_family("family-g-composite", FAMILY_G_COMPOSITE, generated_path);
    generate_family("family-h-layout", FAMILY_H_LAYOUT, generated_path);
    generate_family("family-i-animation", FAMILY_I_ANIMATION, generated_path);
    generate_family("family-j-blocks", FAMILY_J_BLOCKS, generated_path);
    generate_family("family-s-state", FAMILY_S_STATE, generated_path);
    generate_family("family-z-layers", FAMILY_Z_LAYERS, generated_path);

    println!("\n🔧 Step 4: Generating semantic...");
    semantic_generator::generate(generated_path);

    println!("\n🔧 Step 5: Generating themes...");
    theme_generator::generate_themes(generated_path);

    println!("\n🔧 Step 6: Generating root...");
    root_generator::generate(generated_path);

    println!("\n🔧 Step 7: Generating canonrs.css entry...");
    entry_generator::generate(generated_path, styles_path)
        .expect("Failed to generate entry");

    println!("\n🔧 Step 8: Bundling canonrs.bundle.css...");
    bundler::generate(styles_path, bundle_output_path)
        .expect("Failed to bundle");

    println!("\n✅ Complete! All CSS generated.");
}

fn generate_primitives(output_dir: &Path) {
    let mut css = String::from("/* PRIMITIVES - Atomic values */\n:root {\n");
    for token in PRIMITIVE_VALUES {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }
    css.push_str("}\n");

    fs::write(output_dir.join("primitives.css"), css).ok();
    println!("  ✓ primitives.css");
}

fn generate_core(output_dir: &Path) {
    let mut css = String::from("/* FOUNDATION - Core tokens */\n:root {\n");
    for token in CORE_TOKENS {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }
    css.push_str("}\n");

    fs::write(output_dir.join("core.css"), css).ok();
    println!("  ✓ core.css");
}

fn generate_family(name: &str, tokens: &[canonrs_tokens::design::tokens::FamilyToken], output_dir: &Path) {
    let mut css = format!("/* {} */\n[data-theme] {{\n", name);
    for token in tokens {
        css.push_str(&format!("  --{}: {};\n", token.name, token.value));
    }
    css.push_str("}\n");

    fs::write(output_dir.join(format!("{}.css", name)), css).ok();
    println!("  ✓ {}", name);
}
