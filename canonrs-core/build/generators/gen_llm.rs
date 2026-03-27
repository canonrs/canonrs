//! gen_llm.rs — Gera arquivos de contexto para consumo por LLM
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use super::types::{SemanticEntry, BlockInfo};
use super::parsers::{parse_block_props, parse_block_presets, parse_slot_descriptions};

pub fn generate_llm_context(
    semantic: &HashMap<String, SemanticEntry>,
    blocks: &[BlockInfo],
    blocks_dir: &Path,
    layouts_dir: &Path,
    out_dir: &Path,
) {
    generate_llm_components(semantic, out_dir);
    generate_llm_blocks(blocks, blocks_dir, out_dir);
    generate_llm_layouts(blocks, layouts_dir, out_dir);
}

fn generate_llm_components(semantic: &HashMap<String, SemanticEntry>, out_dir: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Componentes UI\n\n");
    out.push_str("> AUTO-GENERATED\n\n");
    out.push_str("---\n\n");
    let mut entries: Vec<&SemanticEntry> = semantic.values().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));
    for entry in &entries {
        out.push_str(&format!("## `{}`\n\n", entry.id));
        out.push_str(&format!("- **Label:** {}\n", entry.label));
        out.push_str(&format!("- **Familia:** {}\n", entry.family));
        out.push_str(&format!("- **Categoria:** {}\n", entry.catalog_category));
        out.push_str(&format!("- **Intencao:** {}\n", entry.intent));
        out.push_str(&format!("- **Descricao:** {}\n", entry.description));
        if entry.capabilities.len() > 0 {
            out.push_str(&format!("- **Capacidades:** {}\n", entry.capabilities.join(", ")));
        }
        out.push_str("\n---\n\n");
    }
    fs::write(out_dir.join("llm_components.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_components.md ({} components)", entries.len());
}

fn generate_llm_blocks(blocks: &[BlockInfo], blocks_dir: &Path, out_dir: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Blocks\n\n");
    out.push_str("> AUTO-GENERATED\n\n");
    out.push_str("---\n\n");
    let block_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "block").collect();
    for block in &block_items {
        let id_snake = block.id.replace('-', "_");
        let src_file = blocks_dir.join(&id_snake).join(format!("{}_block.rs", id_snake));
        let src = match fs::read_to_string(&src_file) { Ok(c) => c, Err(_) => continue };
        out.push_str(&format!("## `{}`\n\n", block.id));
        out.push_str(&format!("- **Categoria:** {}\n", block.category));
        out.push_str(&format!("- **Variante:** {}\n", block.variant));
        out.push_str(&format!("- **Container:** {}\n", if block.container { "sim" } else { "nao" }));
        if block.regions.len() > 0 {
            out.push_str(&format!("- **Regioes:** {}\n", block.regions.join(", ")));
        }
        let props = parse_block_props(&src);
        if props.len() > 0 {
            out.push_str("\n### Props\n\n");
            out.push_str("| Prop | Tipo | Default | Escopo |\n");
            out.push_str("|------|------|---------|--------|\n");
            for prop in &props {
                let default = prop.default.as_deref().unwrap_or("-");
                out.push_str(&format!("| `{}` | {} | `{}` | {} |\n", prop.key, prop.field, default, prop.scope));
            }
        }
        let presets = parse_block_presets(&src);
        if presets.len() > 0 {
            out.push_str("\n### Presets\n\n");
            for preset in &presets {
                let props_str: Vec<String> = preset.props.iter().map(|(k, v)| format!("`{}={}`", k, v)).collect();
                out.push_str(&format!("- **{}:** {}\n", preset.label, props_str.join(", ")));
            }
        }
        out.push_str("\n---\n\n");
    }
    fs::write(out_dir.join("llm_blocks.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_blocks.md ({} blocks)", block_items.len());
}

fn generate_llm_layouts(blocks: &[BlockInfo], layouts_dir: &Path, out_dir: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Layouts\n\n");
    out.push_str("> AUTO-GENERATED\n\n");
    out.push_str("---\n\n");
    let layout_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "layout").collect();
    for layout in &layout_items {
        let id_snake = layout.id.replace('-', "_");
        let src_file = layouts_dir.join(&id_snake).join(format!("{}_layout.rs", id_snake));
        let src = match fs::read_to_string(&src_file) { Ok(c) => c, Err(_) => continue };
        out.push_str(&format!("## `{}`\n\n", layout.id));
        out.push_str(&format!("- **Categoria:** {}\n", layout.category));
        out.push_str(&format!("- **Variante:** {}\n", layout.variant));
        if layout.regions.len() > 0 {
            out.push_str(&format!("- **Regioes:** {}\n", layout.regions.join(", ")));
        }
        let slots = parse_slot_descriptions(&src);
        if slots.len() > 0 {
            out.push_str("\n### Slots\n\n");
            for (id, desc) in &slots {
                out.push_str(&format!("- `{}`: {}\n", id, desc));
            }
        }
        out.push_str("\n---\n\n");
    }
    fs::write(out_dir.join("llm_layouts.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_layouts.md ({} layouts)", layout_items.len());
}
