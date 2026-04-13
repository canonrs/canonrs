//! Generator: block_definitions.rs, layout_definitions.rs

use std::fs;
use std::path::Path;
use super::types::*;
use super::utils::*;
use super::parsers::{parse_block_props, parse_block_presets, extract_canon_field, parse_slot_descriptions};

pub(crate) fn generate_block_definitions(blocks: &[BlockInfo], blocks_dir: &Path, layouts_dir: &Path, out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("#[allow(dead_code, unused_imports)]
");
    code.push_str("use crate::block_types::{BlockDefinition, BlockVariant, BlockCategory, BlockRegion, BlockPropDef, BlockPreset, PropFieldType, PropScope, RegionLayout, AcceptRule};\n");
    code.push_str("// ComponentMeta imported via meta_types\n");

    for b in blocks {
        let dir = if b.kind == "layout" { layouts_dir } else { blocks_dir };
        let dir_name = b.id.replace('-', "_");
        let file_path = dir.join(&dir_name).join(format!("{}_block.rs", dir_name));
        let alt_path  = dir.join(&dir_name).join(format!("{}_layout.rs", dir_name));
        let content = std::fs::read_to_string(&file_path)
            .or_else(|_| std::fs::read_to_string(&alt_path))
            .unwrap_or_default();

        let props   = parse_block_props(&content);
        let presets = parse_block_presets(&content);
        let var_name = to_const_name(&b.id);

        if !props.is_empty() {
            code.push_str(&format!("const {}_PROPS: &[BlockPropDef] = &[\n", var_name));
            for p in &props {
                let field   = prop_field_to_rust(&p.field);
                let default = p.default.as_deref().map(|d| format!("Some(\"{}\")", d)).unwrap_or_else(|| "None".to_string());
                let scope   = match p.scope.as_str() {
                    "structural" => "PropScope::Structural",
                    "visual"     => "PropScope::Visual",
                    "capability" => "PropScope::Capability",
                    _            => "PropScope::Visual",
                };
                let css = p.css.as_deref().map(|c| format!("Some(\"{}\")", c)).unwrap_or_else(|| "None".to_string());
                code.push_str(&format!(
                    "\tBlockPropDef {{ key: \"{key}\", label: \"{label}\", field: {field}, default: {default}, scope: {scope}, css: {css} }},\n",
                    key = p.key, label = p.label, field = field, default = default, scope = scope, css = css,
                ));
            }
            code.push_str("];\n\n");
        }

        if !presets.is_empty() {
            code.push_str(&format!("const {}_PRESETS: &[BlockPreset] = &[\n", var_name));
            for preset in &presets {
                let props_str = preset.props.iter()
                    .map(|(k, v)| format!("(\"{}\", \"{}\")", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                code.push_str(&format!("\tBlockPreset {{ label: \"{label}\", props: &[{props}] }},\n",
                    label = preset.label, props = props_str));
            }
            code.push_str("];\n\n");
        }
    }

        code.push_str("pub static BLOCK_DEFINITIONS_GENERATED: &[BlockDefinition] = &[\n");
    for b in blocks.iter().filter(|b| b.kind == "block") {
        let var_name = to_const_name(&b.id);
        let variant = match b.variant.as_str() {
            "structure" => "BlockVariant::Structure",
            "page"      => "BlockVariant::Page",
            "feature"   => "BlockVariant::Feature",
            "overlay"   => "BlockVariant::Overlay",
            _           => "BlockVariant::Special",
        };
        let category = match b.category.as_str() {
            "layout"  => "BlockCategory::Layout",
            "page"    => "BlockCategory::Page",
            "data"    => "BlockCategory::Data",
            "form"    => "BlockCategory::Form",
            "content" => "BlockCategory::Content",
            "overlay" => "BlockCategory::Overlay",
            _         => "BlockCategory::Layout",
        };
        let regions  = generate_inline_regions(&b.regions);
        let dir_name = b.id.replace('-', "_");
        let block_path = blocks_dir.join(&dir_name).join(format!("{}_block.rs", dir_name));
        let props    = if std::fs::read_to_string(&block_path).map(|c| parse_block_props(&c).len() > 0).unwrap_or(false) { format!("{}_PROPS", var_name) } else { "&[]".to_string() };
        let presets  = if std::fs::read_to_string(&block_path).map(|c| parse_block_presets(&c).len() > 0).unwrap_or(false) { format!("{}_PRESETS", var_name) } else { "&[]".to_string() };
        let meta_var = format!("crate::generated::block_meta::{}_META", var_name);
        let builder_yaml = blocks_dir.join(&dir_name).join("builder.yaml");
        let (req_regions, opt_regions) = read_contract_regions(&builder_yaml);
        code.push_str(&format!(
            "\tBlockDefinition {{ id: \"{id}\", variant: {variant}, category: {category}, is_container: {container}, regions: &{regions}, version: 1, props_schema: {props}, requires_config: false, presets: {presets}, meta: &{meta}, regions_required: &[{req}], regions_optional: &[{opt}] }},\n",
            id = b.id, variant = variant, category = category, container = b.container, regions = regions, props = props, presets = presets, meta = meta_var, req = req_regions, opt = opt_regions,
        ));
    }
    code.push_str("];\n\n");

    code.push_str("pub static LAYOUT_DEFINITIONS_GENERATED: &[BlockDefinition] = &[\n");
    for b in blocks.iter().filter(|b| b.kind == "layout") {
        let var_name = to_const_name(&b.id);
        let regions  = generate_inline_regions(&b.regions);
        let meta_var = format!("crate::generated::block_meta::{}_META", var_name);
        let layout_dir_name = b.id.replace('-', "_").replace("_layout", "");
        let layout_yaml = layouts_dir.join(&layout_dir_name).join("builder.yaml");
        let (req_regions, opt_regions) = read_contract_regions(&layout_yaml);
        code.push_str(&format!(
            "\tBlockDefinition {{ id: \"{id}\", variant: BlockVariant::Page, category: BlockCategory::Layout, is_container: true, regions: &{regions}, version: 1, props_schema: &[], requires_config: false, presets: &[], meta: &{meta}, regions_required: &[{req}], regions_optional: &[{opt}] }},\n",
            id = b.id, regions = regions, meta = meta_var, req = req_regions, opt = opt_regions,
        ));
    }
    code.push_str("];\n");
    std::fs::write(out_dir.join("block_definitions.rs"), code).unwrap();
}

pub(crate) fn generate_layout_definitions(blocks: &[BlockInfo], layouts_dir: &Path, out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("use crate::block_types::{LayoutDefinition, LayoutSlot};\n\n");

    let layouts: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "layout").collect();

    for b in &layouts {
        let var = to_const_name(&b.id);
        let dir_name = b.id.replace('-', "_");
        let files = [
            layouts_dir.join(&dir_name).join(format!("{}_layout.rs", dir_name)),
            layouts_dir.join(&dir_name).join(format!("{}_block.rs", dir_name)),
        ];
        let content = files.iter().find_map(|p| fs::read_to_string(p).ok()).unwrap_or_default();

        let label       = extract_canon_field(&content, "canon-label").unwrap_or_else(|| to_title_case(&b.id));
        let icon        = extract_canon_field(&content, "canon-icon").unwrap_or_else(|| "▭".to_string());
        let description = extract_canon_field(&content, "canon-description").unwrap_or_else(|| format!("{} layout", b.id));
        let slot_descs  = parse_slot_descriptions(&content);

        code.push_str(&format!("const {}_SLOTS: &[LayoutSlot] = &[\n", var));
        for region in &b.regions {
            let desc = slot_descs.iter().find(|(k, _)| k == region).map(|(_, v)| v.as_str()).unwrap_or(region.as_str());
            code.push_str(&format!("    LayoutSlot {{ id: \"{region}\", description: \"{desc}\" }},\n", region = region, desc = desc));
        }
        code.push_str("];\n\n");

        code.push_str(&format!("pub static {}_LAYOUT: LayoutDefinition = LayoutDefinition {{\n", var));
        code.push_str(&format!("    id: \"{}\",\n", b.id));
        code.push_str(&format!("    label: \"{}\",\n", label));
        code.push_str(&format!("    description: \"{}\",\n", description));
        code.push_str(&format!("    icon: \"{}\",\n", icon));
        code.push_str(&format!("    slots: {}_SLOTS,\n", var));
        code.push_str("};\n\n");
    }

    code.push_str("pub static LAYOUT_DEFINITIONS_STATIC: &[LayoutDefinition] = &[\n");
    for b in &layouts {
        code.push_str(&format!("    {}_LAYOUT,\n", to_const_name(&b.id)));
    }
    code.push_str("];\n");

    fs::write(out_dir.join("layout_definitions.rs"), code).unwrap();
}

fn region_accept_rules(region: &str) -> &'static str {
    match region {
        "header"      => "&[AcceptRule::Category(BlockCategory::Page), AcceptRule::Category(BlockCategory::Layout)]",
        "footer"      => "&[AcceptRule::Category(BlockCategory::Page), AcceptRule::Category(BlockCategory::Form)]",
        "sidebar"     => "&[AcceptRule::Category(BlockCategory::Layout), AcceptRule::Category(BlockCategory::Content)]",
        "nav"         => "&[AcceptRule::Category(BlockCategory::Layout)]",
        "actions"     => "&[AcceptRule::Category(BlockCategory::Form), AcceptRule::Category(BlockCategory::Layout)]",
        "filters"     => "&[AcceptRule::Category(BlockCategory::Data), AcceptRule::Category(BlockCategory::Form)]",
        "toolbar"     => "&[AcceptRule::Category(BlockCategory::Layout), AcceptRule::Category(BlockCategory::Form)]",
        "stepper"     => "&[AcceptRule::Category(BlockCategory::Layout)]",
        "steps"       => "&[AcceptRule::Category(BlockCategory::Form)]",
        "pagination"  => "&[AcceptRule::Category(BlockCategory::Data)]",
        "search"      => "&[AcceptRule::Category(BlockCategory::Form)]",
        "results"     => "&[AcceptRule::Category(BlockCategory::Data), AcceptRule::Category(BlockCategory::Content)]",
        "items"       => "&[AcceptRule::Any]",
        "columns"     => "&[AcceptRule::Any]",
        "content"     => "&[AcceptRule::Any]",
        "main"        => "&[AcceptRule::Any]",
        "body"        => "&[AcceptRule::Any]",
        _             => "&[AcceptRule::Any]",
    }
}

fn region_layout(region: &str) -> &'static str {
    match region {
        "columns" => "RegionLayout::Horizontal",
        "nav"     => "RegionLayout::Vertical",
        "sidebar" => "RegionLayout::Vertical",
        "toolbar" => "RegionLayout::Horizontal",
        "actions" => "RegionLayout::Horizontal",
        "filters" => "RegionLayout::Horizontal",
        "stepper" => "RegionLayout::Horizontal",
        _         => "RegionLayout::Vertical",
    }
}

fn generate_inline_regions(regions: &[String]) -> String {
    if regions.is_empty() { return "[]".to_string(); }
    let inner: Vec<String> = regions.iter().map(|r| {
        let accepts = region_accept_rules(r.as_str());
        let layout  = region_layout(r.as_str());
        format!("BlockRegion {{ id: \"{r}\", accepts: {accepts}, layout: {layout}, max_children: None }}",
            r = r, accepts = accepts, layout = layout)
    }).collect();
    format!("[{}]", inner.join(", "))
}

fn read_contract_regions(yaml_path: &std::path::Path) -> (String, String) {
    let content = match std::fs::read_to_string(yaml_path) {
        Ok(c) => c,
        Err(_) => return ("".to_string(), "".to_string()),
    };
    let req = extract_contract_field(&content, "regions_required");
    let opt = extract_contract_field(&content, "regions_optional");
    (req, opt)
}

fn extract_contract_field(content: &str, field: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{}:", field)) {
            let val = trimmed[field.len()+1..].trim();
            let items: Vec<String> = val.split(',')
                .map(|s| s.trim().trim_matches(|c: char| c == '"' || c == '\'' || c == '[' || c == ']').to_string())
                .filter(|s| !s.is_empty())
                .map(|s| format!("\"{}\"", s))
                .collect();
            return items.join(", ");
        }
    }
    "".to_string()
}
