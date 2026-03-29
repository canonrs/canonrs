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
    let blocks_only: Vec<_> = blocks_layouts.iter()
        .filter(|b| !component_ids.contains(&b.id))
        .cloned()
        .collect();

    generate_block_meta(&blocks_only, &out_dir);
    generate_catalog(&semantic, &blocks_layouts, &out_dir, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"));
    generate_layout_definitions(&blocks_layouts, Path::new("../canonrs-server/src/layouts"), &out_dir);
    generate_block_definitions(&blocks_layouts, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"), &out_dir);
    generate_api_files(Path::new("../canonrs-server/src/ui"));
    generate_component_definitions(&semantic, Path::new("../canonrs-server/src/ui"), &out_dir);
    generate_api_files_blocks(Path::new("../canonrs-server/src/blocks"));
    generate_api_files_layouts(Path::new("../canonrs-server/src/layouts"));
    generate_llm_context(&semantic, &blocks_only, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"), &out_dir);

    println!("cargo:warning=CanonRS SSOT: {} primitives, {} blocks/layouts",
        primitives.len(), blocks_layouts.len());
}
