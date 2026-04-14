mod theme_generator;
mod entry_generator;
mod bundler;
mod semantic_generator;
mod root_generator;
mod font_generator;

use canonrs_tokens::design::tokens::components::*;
use canonrs_tokens::design::tokens::layers::LAYERS_TOKENS;
use canonrs_tokens::design::tokens::foundation::*;
use canonrs_tokens::design::tokens::foundation::breakpoints::FOUNDATION_BREAKPOINTS;
use canonrs_tokens::design::tokens::primitives::PRIMITIVE_VALUES;
use std::fs;
use std::path::Path;

fn main() {
    let generated_path = Path::new("../canonrs-server/styles/.generated");
    let styles_path = Path::new("../canonrs-server/styles");
    let bundle_output_path = Path::new("../canonrs-server/styles");

    fs::create_dir_all(generated_path).expect("Failed to create dir");

    println!("🔧 Step 1: Generating primitives...");
    generate_primitives(generated_path);

    println!("\n🔧 Step 1b: Generating fonts (@font-face)...");
    font_generator::generate(generated_path);

    println!("\n🔧 Step 2: Generating foundation...");
    generate_core(generated_path);

    println!("\n🔧 Step 3: Generating components...");
    generate_family("components-overlay",   OVERLAY_TOKENS, generated_path);
    generate_family("components-selection", SELECTION_TOKENS, generated_path);
    generate_family("components-forms", FORMS_TOKENS, generated_path);
    generate_family("components-navigation", NAVIGATION_TOKENS, generated_path);
    generate_family("components-feedback", FEEDBACK_TOKENS, generated_path);
    generate_family("components-data", DATA_TOKENS, generated_path);
    generate_family("components-composite", COMPOSITE_TOKENS, generated_path);
    generate_family("components-layout", LAYOUT_TOKENS, generated_path);
    generate_family("components-animation", ANIMATION_TOKENS, generated_path);
    generate_family("components-blocks", BLOCKS_TOKENS, generated_path);
    generate_family("foundation-interaction", &canonrs_tokens::design::tokens::foundation::interaction::FOUNDATION_INTERACTION, generated_path);
    generate_family("foundation-layers", LAYERS_TOKENS, generated_path);

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
    // Foundation layers
    for token in FOUNDATION_SPACING   { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_RADIUS    { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_MOTION    { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_TYPOGRAPHY{ css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_COLOR     { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_SHADOW    { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_BORDER    { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_INTERACTION{ css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
    for token in FOUNDATION_BREAKPOINTS  { css.push_str(&format!("  --{}: {};\n", token.name, token.value)); }
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
