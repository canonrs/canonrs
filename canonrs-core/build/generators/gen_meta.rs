//! Generator: component_meta.rs, block_meta.rs, mod.rs (generated)

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use super::types::*;
use super::utils::*;

pub(crate) fn generate_component_meta(semantic: &HashMap<String, SemanticEntry>, out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED — do not edit. Edit components.toml instead.\n");
    code.push_str("#[allow(dead_code)]\n");
    code.push_str("use crate::meta_types::{ComponentMeta, ComponentFamily, Capability};\n\n");

    let mut ids: Vec<&String> = semantic.keys().collect();
    ids.sort();

    for id in &ids {
        let s = &semantic[*id];
        let var = to_const_name(id);
        let family = to_family(&s.family);
        let caps = s.capabilities.iter()
            .map(|c| format!("Capability::{}", c))
            .collect::<Vec<_>>()
            .join(", ");
        let req = s.required_parts.iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(", ");
        let opt = s.optional_parts.iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(", ");

        code.push_str(&format!("pub static {}_META: ComponentMeta = ComponentMeta {{\n", var));
        code.push_str(&format!("    id: \"{}\",\n", id));
        code.push_str(&format!("    name: \"{}\",\n", s.label));
        code.push_str(&format!("    family: {},\n", family));
        code.push_str(&format!("    intent: \"{}\",\n", s.intent));
        code.push_str(&format!("    capabilities: &[{}],\n", caps));
        code.push_str(&format!("    composable: {},\n", s.composable));
        code.push_str(&format!("    required_parts: &[{}],\n", req));
        code.push_str(&format!("    optional_parts: &[{}],\n", opt));
        code.push_str("};\n\n");
    }

    code.push_str("pub fn all_meta_generated() -> Vec<&'static ComponentMeta> {\n    vec![\n");
    for id in &ids {
        code.push_str(&format!("        &{}_META,\n", to_const_name(id)));
    }
    code.push_str("    ]\n}\n");

    fs::write(out_dir.join("component_meta.rs"), code).unwrap();
}

pub(crate) fn generate_block_meta(blocks: &[BlockInfo], out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("use crate::meta_types::{ComponentMeta, ComponentFamily};\n\n");

    for b in blocks {
        let var = to_const_name(&b.id);
        let family = match b.category.as_str() {
            "overlay"  => "ComponentFamily::Overlay",
            "form"     => "ComponentFamily::Interactive",
            "data"     => "ComponentFamily::DataDisplay",
            "content"  => "ComponentFamily::DataDisplay",
            "page"     => "ComponentFamily::Layout",
            _          => "ComponentFamily::Layout",
        };
        let name    = to_title_case(&b.id);
        let intent  = b.description.clone()
            .filter(|d| !d.is_empty())
            .unwrap_or_else(|| format!("{} {}", b.label.clone().unwrap_or_else(|| to_title_case(&b.id)), if b.kind == "layout" { "page layout" } else { "block" }));
        let regions: Vec<String> = b.regions.iter().map(|r| format!("\"{}\"", r)).collect();
        let regions_str = regions.join(", ");

        code.push_str(&format!("pub static {}_META: ComponentMeta = ComponentMeta {{\n", var));
        code.push_str(&format!("    id: \"{}\",\n", b.id));
        code.push_str(&format!("    name: \"{}\",\n", name));
        code.push_str(&format!("    family: {},\n", family));
        code.push_str(&format!("    intent: \"{}\",\n", intent));
        code.push_str("    capabilities: &[],\n");
        code.push_str(&format!("    composable: {},\n", b.container));
        code.push_str(&format!("    required_parts: &[{}],\n", regions_str));
        code.push_str("    optional_parts: &[],\n");
        code.push_str("};\n\n");
    }

    fs::write(out_dir.join("block_meta.rs"), code).unwrap();
}

#[allow(dead_code)]
pub(crate) fn generate_mod_update(out_dir: &Path) {
    let code = "// AUTO-GENERATED\npub mod component_meta;\npub mod block_meta;\npub mod block_definitions;\npub mod layout_definitions;\npub mod catalog;\npub mod component_definitions;\npub use component_meta::*;\npub use block_meta::*;\n";
    fs::write(out_dir.join("mod.rs"), code).unwrap();
}
