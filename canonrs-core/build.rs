//! build.rs — CanonRS SSOT Generator
//! SOURCE: primitives/*.rs + blocks/*_block.rs + layouts/*_layout.rs + components.toml
//! GENERATES: schema.json + SSOT_AUDIT.md

use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("cargo:rerun-if-changed=src/primitives");
    println!("cargo:rerun-if-changed=components.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../canonrs-server/src/blocks");
    println!("cargo:rerun-if-changed=../canonrs-server/src/layouts");

    let primitives = parse_primitives(Path::new("src/primitives"));
    let semantic   = parse_semantic(Path::new("components.toml"));
    let blocks_layouts = parse_blocks_and_layouts(
        Path::new("../canonrs-server/src/blocks"),
        Path::new("../canonrs-server/src/layouts"),
    );

    let out_dir = Path::new("src/generated");
    fs::create_dir_all(out_dir).unwrap();

    generate_schema_json(&primitives, &semantic, &blocks_layouts);
    generate_audit(&primitives, &semantic, &blocks_layouts);
    generate_component_meta(&semantic, out_dir);
    let component_ids: std::collections::HashSet<String> = semantic.keys().cloned().collect();
    let blocks_only: Vec<BlockInfo> = blocks_layouts.iter().filter(|b| !component_ids.contains(&b.id)).cloned().collect();
    generate_block_meta(&blocks_only, out_dir);
    generate_catalog(&semantic, &blocks_layouts, out_dir);
    generate_layout_definitions(&blocks_layouts, Path::new("../canonrs-server/src/layouts"), out_dir);
    generate_block_definitions(&blocks_layouts, Path::new("../canonrs-server/src/blocks"), Path::new("../canonrs-server/src/layouts"), out_dir);
    generate_mod_update(out_dir);

    println!("cargo:warning=CanonRS SSOT: {} primitives, {} blocks/layouts",
        primitives.len(), blocks_layouts.len());
}

// ── Primitive Parser ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct PrimitiveInfo {
    pub id:             String,
    pub component_name: String,
    pub behavior:       String,
    pub variants:       Vec<VariantInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct VariantInfo {
    pub enum_name: String,
    pub values:    Vec<String>,
}

fn parse_primitives(dir: &Path) -> HashMap<String, PrimitiveInfo> {
    let mut map = HashMap::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return map,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
        if path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") { continue; }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if let Some(info) = parse_primitive_file(&content) {
            map.insert(info.id.clone(), info);
        }
    }
    map
}

fn parse_primitive_file(content: &str) -> Option<PrimitiveInfo> {
    let component_name = extract_quoted_attr(content, "data-rs-component")?;
    let id             = pascal_to_kebab(&component_name);
    let behavior       = extract_quoted_attr(content, "data-rs-behavior")
        .unwrap_or_else(|| "unknown".to_string());
    let variants       = extract_variants(content);
    Some(PrimitiveInfo { id, component_name, behavior, variants })
}

fn extract_quoted_attr(content: &str, attr: &str) -> Option<String> {
    let needle = format!("{}=\"", attr);
    let start  = content.find(&needle)? + needle.len();
    let end    = content[start..].find('"')? + start;
    Some(content[start..end].to_string())
}

fn extract_variants(content: &str) -> Vec<VariantInfo> {
    let mut variants = vec![];
    let mut in_enum  = false;
    let mut name     = String::new();
    let mut values   = vec![];
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("pub enum ") && !t.contains("State") && !t.contains("Type") {
            in_enum = true;
            name    = t.trim_start_matches("pub enum ").trim_end_matches(" {").to_string();
            values.clear();
        } else if in_enum && t == "}" {
            if !name.is_empty() && !values.is_empty() {
                variants.push(VariantInfo { enum_name: name.clone(), values: values.clone() });
            }
            in_enum = false;
        } else if in_enum && !t.starts_with("//") && !t.starts_with('#') {
            let v = t.trim_end_matches(',');
            if !v.is_empty() && v.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                values.push(v.to_string());
            }
        }
    }
    variants
}

fn pascal_to_kebab(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 { result.push('-'); }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

// ── Semantic Parser ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
struct SemanticEntry {
    pub id:               String,
    pub label:            String,
    pub description:      String,
    pub family:           String,
    pub intent:           String,
    pub capabilities:     Vec<String>,
    #[allow(dead_code)] pub catalog_tags:     Vec<String>,
    pub catalog_category: String,
    #[allow(dead_code)] #[serde(default)] pub required_parts:  Vec<String>,
    #[allow(dead_code)] #[serde(default)] pub optional_parts:  Vec<String>,
    #[allow(dead_code)] #[serde(default)] pub composable:      bool,
    #[allow(dead_code)] #[serde(default)] pub requires_config: bool,
}

#[derive(Debug, serde::Deserialize)]
struct SemanticRegistry { component: Vec<SemanticEntry> }

fn parse_semantic(path: &Path) -> HashMap<String, SemanticEntry> {
    let mut map = HashMap::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return map,
    };
    let registry: SemanticRegistry = match toml::from_str(&content) {
        Ok(r) => r,
        Err(e) => { println!("cargo:warning=components.toml error: {}", e); return map; }
    };
    for entry in registry.component { map.insert(entry.id.clone(), entry); }
    map
}

// ── Block/Layout Parser ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct BlockInfo {
    pub id:          String,
    pub kind:        String,
    pub category:    String,
    pub variant:     String,
    pub container:   bool,
    pub regions:     Vec<String>,
    #[allow(dead_code)]
    pub label:       Option<String>,
    #[allow(dead_code)]
    pub description: Option<String>,
    #[allow(dead_code)]
    pub tags:        Vec<String>,
}

fn parse_blocks_and_layouts(blocks_dir: &Path, layouts_dir: &Path) -> Vec<BlockInfo> {
    let mut result = vec![];
    for (dir, kind) in &[(blocks_dir, "block"), (layouts_dir, "layout")] {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() { continue; }
            let files = match fs::read_dir(&path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            for file in files.flatten() {
                let fpath = file.path();
                let fname = fpath.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !fname.ends_with(".rs") || fname == "mod.rs"
                    || fname.contains("example") || fname.contains("mock") { continue; }
                let content = match fs::read_to_string(&fpath) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                if !content.contains("@canon-id:") { continue; }
                if let Some(info) = parse_canon_header(&content, kind) {
                    result.push(info);
                }
            }
        }
    }
    result
}

fn parse_canon_header(content: &str, kind: &str) -> Option<BlockInfo> {
    let id        = extract_canon_field(content, "canon-id")?;
    let category  = extract_canon_field(content, "canon-category").unwrap_or_else(|| "layout".to_string());
    let variant   = extract_canon_field(content, "canon-variant").unwrap_or_else(|| "structure".to_string());
    let container = extract_canon_field(content, "canon-container").map(|v| v == "true").unwrap_or(false);
    let regions   = extract_canon_field(content, "canon-regions")
        .map(|r| r.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    let label       = extract_canon_field(content, "canon-label");
    let description = extract_canon_field(content, "canon-description");
    let tags        = extract_canon_field(content, "canon-tags")
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    Some(BlockInfo { id, kind: kind.to_string(), category, variant, container, regions, label, description, tags })
}

fn extract_canon_field(content: &str, field: &str) -> Option<String> {
    let needle = format!("//! @{}:", field);
    let line   = content.lines().find(|l| l.contains(&needle))?;
    Some(line.splitn(2, &needle).nth(1)?.trim().to_string())
}

// ── Generators ────────────────────────────────────────────────────────────────

fn generate_schema_json(
    primitives: &HashMap<String, PrimitiveInfo>,
    semantic:   &HashMap<String, SemanticEntry>,
    blocks:     &[BlockInfo],
) {
    let mut ids: Vec<&String> = primitives.keys().collect();
    ids.sort();

    let components: Vec<serde_json::Value> = ids.iter().map(|id| {
        let prim = &primitives[*id];
        let sem  = semantic.get(*id);
        serde_json::json!({
            "id": id, "type": "component",
            "component_name": prim.component_name,
            "behavior": prim.behavior,
            "variants": prim.variants.iter().map(|v| serde_json::json!({"name": v.enum_name, "values": v.values})).collect::<Vec<_>>(),
            "label": sem.map(|s| &s.label),
            "description": sem.map(|s| &s.description),
            "family": sem.map(|s| &s.family),
            "intent": sem.map(|s| &s.intent),
            "capabilities": sem.map(|s| &s.capabilities),
            "catalog_tags": sem.map(|s| &s.catalog_tags),
            "catalog_category": sem.map(|s| &s.catalog_category),
            "has_semantic": sem.is_some(),
        })
    }).collect();

    let mut block_entries: Vec<serde_json::Value> = blocks.iter().map(|b| {
        serde_json::json!({
            "id": b.id, "type": b.kind,
            "category": b.category,
            "variant": b.variant,
            "container": b.container,
            "regions": b.regions,
        })
    }).collect();
    block_entries.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));

    let schema = serde_json::json!({
        "components": components,
        "blocks": block_entries,
    });

    fs::write("schema.json", serde_json::to_string_pretty(&schema).unwrap()).unwrap();
}

fn generate_audit(
    primitives: &HashMap<String, PrimitiveInfo>,
    semantic:   &HashMap<String, SemanticEntry>,
    blocks:     &[BlockInfo],
) {
    let mut missing_sem: Vec<&String> = primitives.keys()
        .filter(|id| !semantic.contains_key(*id)).collect();
    missing_sem.sort();

    let mut orphan_sem: Vec<&String> = semantic.keys()
        .filter(|id| !primitives.contains_key(*id)).collect();
    orphan_sem.sort();

    let mut report = String::from("# CanonRS SSOT Audit\n\n## Components\n");
    report.push_str(&format!("- Primitives: {}\n", primitives.len()));
    report.push_str(&format!("- Semantic (components.toml): {}\n", semantic.len()));
    report.push_str(&format!("- Complete: {}\n", primitives.keys().filter(|id| semantic.contains_key(*id)).count()));
    report.push_str(&format!("- Missing semantic: {}\n", missing_sem.len()));
    report.push_str(&format!("- Orphan semantic: {}\n\n", orphan_sem.len()));

    if !missing_sem.is_empty() {
        report.push_str("### Missing semantic\n");
        for id in &missing_sem { report.push_str(&format!("- `{}`\n", id)); }
    }
    if !orphan_sem.is_empty() {
        report.push_str("\n### Orphan semantic\n");
        for id in &orphan_sem { report.push_str(&format!("- `{}`\n", id)); }
    }

    let block_count   = blocks.iter().filter(|b| b.kind == "block").count();
    let layout_count  = blocks.iter().filter(|b| b.kind == "layout").count();
    report.push_str(&format!("\n## Blocks\n- Total: {}\n\n## Layouts\n- Total: {}\n\n", block_count, layout_count));

    report.push_str("### Block list\n");
    let mut sorted = blocks.to_vec();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    for b in &sorted {
        report.push_str(&format!("- `{}` ({}) regions: [{}]\n", b.id, b.kind, b.regions.join(", ")));
    }

    fs::write("SSOT_AUDIT.md", report).unwrap();
}


// ── Code Generators ───────────────────────────────────────────────────────────

fn generate_component_meta(semantic: &HashMap<String, SemanticEntry>, out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED — do not edit. Edit components.toml instead.\n");
    code.push_str("#![allow(dead_code)]\n");
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


fn to_const_name(id: &str) -> String {
    id.replace('-', "_").to_uppercase()
}

fn to_family(family: &str) -> &'static str {
    match family {
        "overlay"      => "ComponentFamily::Overlay",
        "input"        => "ComponentFamily::Input",
        "feedback"     => "ComponentFamily::Feedback",
        "navigation"   => "ComponentFamily::Navigation",
        "layout"       => "ComponentFamily::Layout",
        "data_display" => "ComponentFamily::DataDisplay",
        "typography"   => "ComponentFamily::Typography",
        "interactive"  => "ComponentFamily::Interactive",
        _              => "ComponentFamily::Utility",
    }
}


fn generate_block_meta(blocks: &[BlockInfo], out_dir: &Path) {
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
        let name = to_title_case(&b.id);
        let kind_str = if b.kind == "layout" { "page layout" } else { "block" };
        let intent = format!("{} {}", b.id.replace('-', " "), kind_str);

        code.push_str(&format!("pub static {}_META: ComponentMeta = ComponentMeta {{\n", var));
        code.push_str(&format!("    id: \"{}\",\n", b.id));
        code.push_str(&format!("    name: \"{}\",\n", name));
        code.push_str(&format!("    family: {},\n", family));
        code.push_str(&format!("    intent: \"{}\",\n", intent));
        code.push_str("    capabilities: &[],\n");
        code.push_str(&format!("    composable: {},\n", b.container));
        code.push_str("    required_parts: &[],\n");
        code.push_str("    optional_parts: &[],\n");
        code.push_str("};\n\n");
    }

    fs::write(out_dir.join("block_meta.rs"), code).unwrap();
}


fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}


// ── Extended Block Info with Props ───────────────────────────────────────────

#[derive(Debug, Clone)]
struct PropInfo {
    pub key:     String,
    pub label:   String,
    pub field:   String,
    pub default: Option<String>,
    pub scope:   String,
    pub css:     Option<String>,
}

#[derive(Debug, Clone)]
struct PresetInfo {
    pub label: String,
    pub props: Vec<(String, String)>,
}

fn parse_block_props(content: &str) -> Vec<PropInfo> {
    let mut props = vec![];
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with("//! @canon-prop:") { continue; }
        let val = t.trim_start_matches("//! @canon-prop:").trim();
        let parts: Vec<&str> = val.splitn(6, '|').map(|p| p.trim()).collect();
        if parts.len() < 4 { continue; }
        let key     = parts[0].to_string();
        let field   = parts[1].to_string();
        let default = if parts[2].is_empty() { None } else { Some(parts[2].to_string()) };
        let scope   = parts[3].to_string();
        let css     = if parts.len() > 4 && !parts[4].is_empty() { Some(parts[4].to_string()) } else { None };
        let label   = key.replace('-', " ")
            .split_whitespace()
            .map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() })
            .collect::<Vec<_>>()
            .join(" ");
        props.push(PropInfo { key, label, field, default, scope, css });
    }
    props
}

fn parse_block_presets(content: &str) -> Vec<PresetInfo> {
    let mut presets = vec![];
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with("//! @canon-preset:") { continue; }
        let val = t.trim_start_matches("//! @canon-preset:").trim();
        let parts: Vec<&str> = val.splitn(2, '|').map(|p| p.trim()).collect();
        if parts.len() < 2 { continue; }
        let label = parts[0].to_string();
        let props: Vec<(String, String)> = parts[1].split(',')
            .filter_map(|p| {
                let kv: Vec<&str> = p.splitn(2, '=').collect();
                if kv.len() == 2 { Some((kv[0].trim().to_string(), kv[1].trim().to_string())) }
                else { None }
            })
            .collect();
        presets.push(PresetInfo { label, props });
    }
    presets
}

fn prop_field_to_rust(field: &str) -> String {
    if field.starts_with("Select(") {
        let inner = field.trim_start_matches("Select(").trim_end_matches(')');
        let opts: Vec<String> = inner.split(',')
            .map(|o| {
                let kv: Vec<&str> = o.splitn(2, ':').collect();
                if kv.len() == 2 { format!("(\"{}\", \"{}\")", kv[0].trim(), kv[1].trim()) }
                else { format!("(\"{}\", \"{}\")", o.trim(), o.trim()) }
            })
            .collect();
        // Use a leaked static for the slice
        format!("PropFieldType::Select(&[{}])", opts.join(", "))
    } else {
        match field {
            "Text"   => "PropFieldType::Text".to_string(),
            "Number" => "PropFieldType::Number".to_string(),
            "Toggle" => "PropFieldType::Toggle".to_string(),
            "Color"  => "PropFieldType::Color".to_string(),
            _        => "PropFieldType::Text".to_string(),
        }
    }
}

fn generate_block_definitions(blocks: &[BlockInfo], blocks_dir: &std::path::Path, layouts_dir: &std::path::Path, out_dir: &std::path::Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("#![allow(dead_code, unused_imports)]\n");
    code.push_str("use crate::block_types::{BlockDefinition, BlockVariant, BlockCategory, BlockRegion, BlockPropDef, BlockPreset, PropFieldType, PropScope, RegionLayout, AcceptRule};\n");
    code.push_str("use crate::meta::ComponentMeta;\n\n");

    for b in blocks {
        let dir = if b.kind == "layout" { layouts_dir } else { blocks_dir };
        let dir_name = b.id.replace('-', "_");
        let file_path = dir.join(&dir_name).join(format!("{}_block.rs", dir_name))
            .to_string_lossy().to_string();
        let alt_path = dir.join(&dir_name).join(format!("{}_layout.rs", dir_name))
            .to_string_lossy().to_string();

        let content = std::fs::read_to_string(&file_path)
            .or_else(|_| std::fs::read_to_string(&alt_path))
            .unwrap_or_default();

        let props = parse_block_props(&content);
        let presets = parse_block_presets(&content);
        let var_name = to_const_name(&b.id);

        // Generate props const
        if !props.is_empty() {
            code.push_str(&format!("const {}_PROPS: &[BlockPropDef] = &[\n", var_name));
            for p in &props {
                let field = prop_field_to_rust(&p.field);
                let default = p.default.as_deref()
                    .map(|d| format!("Some(\"{}\")", d))
                    .unwrap_or_else(|| "None".to_string());
                let scope = match p.scope.as_str() {
                    "structural" => "PropScope::Structural",
                    "visual"     => "PropScope::Visual",
                    "capability" => "PropScope::Capability",
                    _            => "PropScope::Visual",
                };
                let css = p.css.as_deref()
                    .map(|c| format!("Some(\"{}\")", c))
                    .unwrap_or_else(|| "None".to_string());
                code.push_str(&format!(
                    "\tBlockPropDef {{ key: \"{key}\", label: \"{label}\", field: {field}, default: {default}, scope: {scope}, css: {css} }},\n",
                    key = p.key, label = p.label, field = field, default = default, scope = scope, css = css,
                ));
            }
            code.push_str("];\n\n");
        }

        // Generate presets const
        if !presets.is_empty() {
            code.push_str(&format!("const {}_PRESETS: &[BlockPreset] = &[\n", var_name));
            for preset in &presets {
                let props_str = preset.props.iter()
                    .map(|(k, v)| format!("(\"{}\", \"{}\")", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                code.push_str(&format!("\tBlockPreset {{ label: \"{}\", props: &[{}] }},\n", preset.label, props_str));
            }
            code.push_str("];\n\n");
        }
    }

    // Generate regions and BLOCK_DEFINITIONS
    code.push_str("// ── Region helpers ───────────────────────────────────────────────────────────\n");
    code.push_str("const ANY: &[AcceptRule] = &[AcceptRule::Any];\n\n");

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
        let regions = generate_inline_regions(&b.regions);
        let props = if std::fs::read_to_string(
            blocks_dir.join(b.id.replace('-', "_")).join(format!("{}_block.rs", b.id.replace('-', "_")))
        ).map(|c| parse_block_props(&c).len() > 0).unwrap_or(false) {
            format!("{}_PROPS", var_name)
        } else { "&[]".to_string() };
        let presets = if std::fs::read_to_string(
            blocks_dir.join(b.id.replace('-', "_")).join(format!("{}_block.rs", b.id.replace('-', "_")))
        ).map(|c| parse_block_presets(&c).len() > 0).unwrap_or(false) {
            format!("{}_PRESETS", var_name)
        } else { "&[]".to_string() };
        let meta_var = format!("crate::meta::{}_META", var_name);

        code.push_str(&format!(
            "\tBlockDefinition {{ id: \"{id}\", variant: {variant}, category: {category}, is_container: {container}, regions: &{regions}, version: 1, props_schema: {props}, requires_config: false, presets: {presets}, meta: &{meta} }},\n",
            id = b.id, variant = variant, category = category,
            container = b.container, regions = regions,
            props = props, presets = presets, meta = meta_var,
        ));
    }
    code.push_str("];\n\n");

    code.push_str("pub static LAYOUT_DEFINITIONS_GENERATED: &[BlockDefinition] = &[\n");
    for b in blocks.iter().filter(|b| b.kind == "layout") {
        let var_name = to_const_name(&b.id);
        let regions = generate_inline_regions(&b.regions);
        let meta_var = format!("crate::meta::{}_META", var_name);
        code.push_str(&format!(
            "\tBlockDefinition {{ id: \"{id}\", variant: BlockVariant::Page, category: BlockCategory::Layout, is_container: true, regions: &{regions}, version: 1, props_schema: &[], requires_config: false, presets: &[], meta: &{meta} }},\n",
            id = b.id, regions = regions, meta = meta_var,
        ));
    }
    code.push_str("];\n");

    std::fs::write(out_dir.join("block_definitions.rs"), code).unwrap();
}

fn generate_inline_regions(regions: &[String]) -> String {
    if regions.is_empty() { return "[]".to_string(); }
    let inner: Vec<String> = regions.iter().map(|r| {
        format!(
            "BlockRegion {{ id: \"{r}\", accepts: ANY, layout: RegionLayout::Vertical, max_children: None }}",
            r = r
        )
    }).collect();
    format!("[{}]", inner.join(", "))
}


fn generate_mod_update(out_dir: &Path) {
    let code = "// AUTO-GENERATED\npub mod component_meta;\npub mod block_meta;\npub mod block_definitions;\npub mod layout_definitions;\npub mod catalog;\npub use component_meta::*;\npub use block_meta::*;\n";
    fs::write(out_dir.join("mod.rs"), code).unwrap();
}

fn parse_slot_descriptions(content: &str) -> Vec<(String, String)> {
    let needle = "//! @canon-slot-descriptions:";
    let line = match content.lines().find(|l| l.contains(needle)) {
        Some(l) => l,
        None => return vec![],
    };
    let val = line.splitn(2, needle).nth(1).unwrap_or("").trim();
    val.split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.splitn(2, ':').collect();
            if parts.len() == 2 { Some((parts[0].trim().to_string(), parts[1].trim().to_string())) }
            else { None }
        })
        .collect()
}

fn generate_layout_definitions(blocks: &[BlockInfo], layouts_dir: &Path, out_dir: &Path) {
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
        let content = files.iter()
            .find_map(|p| fs::read_to_string(p).ok())
            .unwrap_or_default();

        let label = extract_canon_field(&content, "canon-label").unwrap_or_else(|| to_title_case(&b.id));
        let icon = extract_canon_field(&content, "canon-icon").unwrap_or_else(|| "▭".to_string());
        let description = extract_canon_field(&content, "canon-description")
            .unwrap_or_else(|| format!("{} layout", b.id));
        let slot_descs = parse_slot_descriptions(&content);

        // Generate slots const
        code.push_str(&format!("const {}_SLOTS: &[LayoutSlot] = &[\n", var));
        for region in &b.regions {
            let desc = slot_descs.iter()
                .find(|(k, _)| k == region)
                .map(|(_, v)| v.as_str())
                .unwrap_or(region.as_str());
            code.push_str(&format!("    LayoutSlot {{ id: \"{region}\", description: \"{desc}\" }},\n",
                region = region, desc = desc));
        }
        code.push_str("];

");

        // Generate LayoutDefinition static
        code.push_str(&format!("pub static {}_LAYOUT: LayoutDefinition = LayoutDefinition {{\n", var));
        code.push_str(&format!("    id: \"{}\",\n", b.id));
        code.push_str(&format!("    label: \"{}\",\n", label));
        code.push_str(&format!("    description: \"{}\",\n", description));
        code.push_str(&format!("    icon: \"{}\",\n", icon));
        code.push_str(&format!("    slots: {}_SLOTS,\n", var));
        code.push_str("};\n\n");
    }

    // Generate LAYOUT_DEFINITIONS_STATIC
    code.push_str("pub static LAYOUT_DEFINITIONS_STATIC: &[LayoutDefinition] = &[\n");
    for b in &layouts {
        let var = to_const_name(&b.id);
        code.push_str(&format!("    {}_LAYOUT,\n", var));
    }
    code.push_str("];\n");

    fs::write(out_dir.join("layout_definitions.rs"), code).unwrap();
}

fn generate_catalog(semantic: &HashMap<String, SemanticEntry>, blocks: &[BlockInfo], out_dir: &Path) {
    let mut code = String::new();
    code.push_str("// AUTO-GENERATED\n");
    code.push_str("use crate::catalog_types::{CatalogEntry, CatalogCategory};\n\n");
    code.push_str("pub static CATALOG_GENERATED: &[CatalogEntry] = &[\n");

    let mut ids: Vec<&String> = semantic.keys().collect();
    ids.sort();
    for id in &ids {
        let s = &semantic[*id];
        let cat = to_catalog_category(&s.catalog_category);
        let tags: Vec<String> = s.catalog_tags.iter().map(|t| format!("\"{}\"", t)).collect();
        let tags_str = tags.join(", ");
        code.push_str("    CatalogEntry { ");
        code.push_str(&format!("id: \"{}\", ", id));
        code.push_str(&format!("label: \"{}\", ", s.label));
        code.push_str(&format!("description: \"{}\", ", s.description));
        code.push_str(&format!("category: {}, ", cat));
        code.push_str(&format!("tags: &[{}]", tags_str));
        code.push_str(", parts: None, regions: &[], accepts: &[]");
        code.push_str(" },\n");
    }

    for b in blocks.iter().filter(|b| b.kind == "block") {
        let label = b.label.clone().unwrap_or_else(|| to_title_case(&b.id));
        let desc = b.description.clone().unwrap_or_else(|| format!("{} block", label));
        let tags: Vec<String> = if b.tags.is_empty() {
            vec![b.id.clone()]
        } else {
            b.tags.clone()
        };
        let tags_str = tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ");
        code.push_str("    CatalogEntry { ");
        code.push_str(&format!("id: \"block.{}\", ", b.id));
        code.push_str(&format!("label: \"{}\", ", label));
        code.push_str(&format!("description: \"{}\", ", desc));
        code.push_str("category: CatalogCategory::Layout, ");
        code.push_str(&format!("tags: &[{}]", tags_str));
        code.push_str(", parts: None, regions: &[], accepts: &[]");
        code.push_str(" },\n");
    }

    for b in blocks.iter().filter(|b| b.kind == "layout") {
        let label = b.label.clone().unwrap_or_else(|| to_title_case(&b.id));
        let desc = b.description.clone().unwrap_or_else(|| format!("{} layout", label));
        let tags: Vec<String> = if b.tags.is_empty() {
            vec![b.id.clone()]
        } else {
            b.tags.clone()
        };
        let tags_str = tags.iter().map(|t| format!("\"{}\"", t)).collect::<Vec<_>>().join(", ");
        code.push_str("    CatalogEntry { ");
        code.push_str(&format!("id: \"layout.{}\", ", b.id));
        code.push_str(&format!("label: \"{}\", ", label));
        code.push_str(&format!("description: \"{}\", ", desc));
        code.push_str("category: CatalogCategory::Layout, ");
        code.push_str(&format!("tags: &[{}]", tags_str));
        code.push_str(", parts: None, regions: &[], accepts: &[]");
        code.push_str(" },\n");
    }

    code.push_str("];\n");
    fs::write(out_dir.join("catalog.rs"), code).unwrap();
}


fn to_catalog_category(cat: &str) -> &'static str {
    match cat {
        "Action"     => "CatalogCategory::Action",
        "Display"    => "CatalogCategory::Display",
        "Feedback"   => "CatalogCategory::Feedback",
        "Form"       => "CatalogCategory::Form",
        "Navigation" => "CatalogCategory::Navigation",
        "Overlay"    => "CatalogCategory::Overlay",
        "Data"       => "CatalogCategory::Data",
        "Layout"     => "CatalogCategory::Layout",
        _            => "CatalogCategory::Display",
    }
}
