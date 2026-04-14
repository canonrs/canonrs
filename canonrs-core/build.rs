//! build.rs — CanonRS SSOT Generator
//! SOURCE: primitives/*.rs + blocks/*_block.rs + layouts/*_layout.rs + components.toml
//! GENERATES: schema.json + SSOT_AUDIT.md + OUT_DIR/generated/*

mod build {
    pub(crate) mod types;
    pub(crate) mod utils;
    pub(crate) mod parsers;
    pub(crate) mod generators;
}

use std::path::Path;
use build::parsers::*;
use build::generators::*;

fn main() {
    println!("cargo:rerun-if-changed=src/primitives");
    println!("cargo:rerun-if-changed=../canonrs-server/src/ui");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../canonrs-server/src/blocks");
    println!("cargo:rerun-if-changed=../canonrs-server/src/layouts");
    println!("cargo:rerun-if-changed=build");

    let primitives     = parse_primitives(Path::new("src/primitives"));
    let semantic       = parse_ui_components_semantic(Path::new("../canonrs-server/src/ui"));
    let blocks_layouts = parse_blocks_and_layouts(
        Path::new("../canonrs-server/src/blocks"),
        Path::new("../canonrs-server/src/layouts"),
    );

    let out_dir_base = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_base).join("generated");
    std::fs::create_dir_all(&out_dir).unwrap();

    generate_schema_json(&primitives, &semantic, &blocks_layouts);
    generate_audit(&primitives, &semantic, &blocks_layouts);
    generate_component_meta(&semantic, &out_dir);

    let component_ids: std::collections::HashSet<String> = semantic.keys().cloned().collect();
    let _blocks_only: Vec<_> = blocks_layouts.iter()
        .filter(|b| !component_ids.contains(&b.id))
        .cloned()
        .collect();

    generate_block_meta(&blocks_layouts, &out_dir);
    generate_catalog(&semantic, &blocks_layouts, &out_dir, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"));
    generate_layout_definitions(&blocks_layouts, Path::new("../canonrs-server/src/layouts"), &out_dir);
    generate_block_definitions(&blocks_layouts, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"), &out_dir);
    generate_api_files(Path::new("../canonrs-server/src/ui"));
    generate_component_definitions(&semantic, Path::new("../canonrs-server/src/ui"), &out_dir);
    generate_api_files_blocks(Path::new("../canonrs-server/src/blocks"));
    generate_api_files_layouts(Path::new("../canonrs-server/src/layouts"));
    generate_llm_context(&semantic, &blocks_layouts, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"), &out_dir);

    // Canon Rules — parse + generate
    let rules_dir = Path::new("../canonrs-rules");
    if rules_dir.exists() {
        println!("cargo:rerun-if-changed=../canonrs-rules");
        let rules = parse_rules(rules_dir);
        let rules_out = Path::new("src/generated");
        generate_rules_json(&rules, &rules_out.join("rules.json"));
        generate_rules_seo(&rules, &rules_out.join("rules_seo.json"));
        generate_rules_llm(&rules, &rules_out.join("rules_llm.md"));
    }

    // Showcase data
    let showcase_out = Path::new("src/generated/showcase.json");
    generate_showcase(Path::new("../canonrs-server/src/ui"), showcase_out);

    println!("cargo:warning=CanonRS SSOT: {} primitives, {} blocks/layouts",
        primitives.len(), blocks_layouts.len());

    // CR-VALIDATOR: roda validate_components.py e emite warnings/erros no cargo build
    println!("cargo:rerun-if-changed=../canonrs-tokens/bin/validate_components.py");
    println!("cargo:rerun-if-changed=../canonrs-server/styles/ui");
    let validator_path = Path::new("../canonrs-tokens/bin/validate_components.py");
    if validator_path.exists() {
        let output = std::process::Command::new("python3")
            .arg(validator_path)
            .current_dir("../canonrs-tokens")
            .output();
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let _stderr = String::from_utf8_lossy(&out.stderr);
                // emitir cada violation como cargo warning
                for line in stdout.lines() {
                    if line.contains("[ERRO]") || line.contains("[CR-") || line.contains("[INEXISTENTE]") || line.contains("[CONTRATO]") {
                        println!("cargo:warning=CANON: {}", line.trim());
                    }
                }
                if !out.status.success() {
                    // contar violations
                    let violations: Vec<&str> = stdout.lines()
                        .filter(|l| l.contains("violations found"))
                        .collect();
                    if let Some(v) = violations.first() {
                        panic!("CanonRS validator FAILED: {}", v.trim());
                    }
                    panic!("CanonRS validator FAILED — fix violations before building");
                } else {
                    println!("cargo:warning=CANON: ✓ 88 components clean");
                }
            }
            Err(e) => {
                println!("cargo:warning=CANON: validator not found ({})", e);
            }
        }
    }
}
