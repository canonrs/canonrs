//! gen_llm.rs — Generates LLM context files
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use super::types::{SemanticEntry, BlockInfo};
use super::parsers::{parse_block_props, parse_block_presets, parse_slot_descriptions, parse_slot_accepts, extract_canon_field};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
struct BuilderYaml {
    id:                Option<String>,
    label:             Option<String>,
    category:          Option<String>,
    description:       Option<String>,
    intent:            Option<String>,
    keywords:          Option<String>,
    pain:              Option<String>,
    promise:           Option<String>,
    why:               Option<String>,
    before:            Option<String>,
    after:             Option<String>,
    rules:             Option<Vec<String>>,
    use_cases:         Option<Vec<String>>,
    related:           Option<Vec<String>>,
    badges:            Option<Vec<String>>,
    pillar:            Option<String>,
    tags:              Option<Vec<String>>,
    slot_accepts:      Option<HashMap<String, String>>,
    contract:          Option<HashMap<String, serde_yaml::Value>>,
    required_parts:    Option<Vec<String>>,
    optional_parts:    Option<Vec<String>>,
    states:            Option<Vec<String>>,
    boundary_type:     Option<String>,
    island:            Option<String>,
}

fn read_builder_yaml(dir: &Path) -> BuilderYaml {
    let yaml_path = dir.join("builder.yaml");
    let content = match fs::read_to_string(&yaml_path) {
        Ok(c) => c,
        Err(_) => return BuilderYaml::default(),
    };
    serde_yaml::from_str(&content).unwrap_or_default()
}

pub(crate) fn generate_llm_context(
    semantic: &HashMap<String, SemanticEntry>,
    blocks: &[BlockInfo],
    blocks_dir: &Path,
    layouts_dir: &Path,
    _out_dir: &Path,
) {
    // Escreve sempre em src/generated — fonte de verdade versionável
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let src_generated = std::path::Path::new(&manifest).join("src/generated");
    std::fs::create_dir_all(&src_generated).ok();

    let ui_dir = blocks_dir.parent().map(|p| p.join("ui")).unwrap_or_default();

    let components_md = build_llm_components(semantic, &ui_dir);
    let blocks_md     = build_llm_blocks(blocks, blocks_dir);
    let layouts_md    = build_llm_layouts(blocks, layouts_dir);
    let primitives_md = build_llm_layout_primitives();

    let mut unified = String::new();
    unified.push_str("# CanonRS — LLM Context\n\n");
    unified.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    unified.push_str("---\n\n");
    unified.push_str("# Critical Syntax Rules\n\n");
    unified.push_str("## Slot Syntax — Layouts vs Blocks\n\n");
    unified.push_str("Layout slots use `leptos::children::ToChildren::to_children()` — NOT `slot!()`:\n\n");
    unified.push_str("```rust\n");
    unified.push_str("// LAYOUTS — ToChildren::to_children()\n");
    unified.push_str("content=leptos::children::ToChildren::to_children(|| view! { <MyComponent/> })\n\n");
    unified.push_str("// BLOCKS — slot!() macro\n");
    unified.push_str("body=slot!(|| view! { <MyComponent/> }.into_any())\n");
    unified.push_str("```\n\n");
    unified.push_str("## Import Paths\n\n");
    unified.push_str("```rust\n");
    unified.push_str("// Layout primitives — from canonrs:: root\n");
    unified.push_str("use canonrs::{StackPrimitive, StackGap, FlexPrimitive, FlexJustify, FlexAlign, GridPrimitive, GridCols, GridGap, SpacerPrimitive};\n\n");
    unified.push_str("// Container — exception\n");
    unified.push_str("use canonrs::layout::container::{ContainerPrimitive, ContainerSize};\n\n");
    unified.push_str("// Top-level enums\n");
    unified.push_str("use canonrs::{BadgeVariant, ActivityState, InlineNoticeVariant, ToggleState};\n");
    unified.push_str("```\n\n");
    unified.push_str("## BadgeVariant values\n\n");
    unified.push_str("Default, Primary, Success, Warning, Destructive, Outline\n");
    unified.push_str("DOES NOT EXIST: Secondary — use Outline\n\n");
    unified.push_str("## Type Inference — E0282\n\n");
    unified.push_str("Complex layout slots cause E0282. Always extract to #[component]:\n\n");
    unified.push_str("```rust\n");
    unified.push_str("#[component] fn MyContent() -> impl IntoView { view! { <Stack>...</Stack> } }\n");
    unified.push_str("content=leptos::children::ToChildren::to_children(|| view! { <MyContent/> })\n");
    unified.push_str("```\n\n");
    unified.push_str("## Primitive nesting — always wrap UI in Stack/Grid/Flex inside block slots\n\n");
    unified.push_str("```rust\n");
    unified.push_str("// CORRECT\n");
    unified.push_str("body=slot!(|| view! { <StackPrimitive gap=StackGap::Md><Alert .../><Progress .../></StackPrimitive> }.into_any())\n");
    unified.push_str("// WRONG — UI directly in slot without primitive\n");
    unified.push_str("body=slot!(|| view! { <Alert .../><Progress .../> }.into_any())\n");
    unified.push_str("```\n\n");
    unified.push_str("---\n\n");
    unified.push_str(&components_md);
    unified.push_str(&blocks_md);
    unified.push_str(&layouts_md);
    unified.push_str(&primitives_md);

    fs::write(src_generated.join("llm_context.md"), &unified).unwrap();

    let total = semantic.len()
        + blocks.iter().filter(|b| b.kind == "block").count()
        + blocks.iter().filter(|b| b.kind == "layout").count()
        + 6;
    println!("cargo:warning=CanonRS LLM: llm_context.md ({} entries)", total);
}

fn build_llm_components(semantic: &HashMap<String, SemanticEntry>, ui_dir: &Path) -> String {
    let mut out = String::new();
    out.push_str("# CanonRS — UI Components\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let mut entries: Vec<&SemanticEntry> = semantic.values().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));

    for entry in &entries {
        let id_snake = entry.id.replace('-', "_");
        let yaml = read_builder_yaml(&ui_dir.join(&id_snake));

        out.push_str(&format!("## `{}`\n\n", entry.id));
        out.push_str(&format!("- **Label:** {}\n", entry.label));
        out.push_str(&format!("- **Category:** {}\n", entry.catalog_category));
        out.push_str(&format!("- **Intent:** {}\n", entry.intent));
        out.push_str(&format!("- **Description:** {}\n", entry.description));

        if let Some(keywords) = &yaml.keywords {
            if !keywords.is_empty() {
                out.push_str(&format!("- **Keywords:** {}\n", keywords));
            }
        }
        if let Some(pain) = &yaml.pain {
            if !pain.is_empty() {
                out.push_str(&format!("- **Pain:** {}\n", pain));
            }
        }
        if let Some(promise) = &yaml.promise {
            if !promise.is_empty() {
                out.push_str(&format!("- **Promise:** {}\n", promise));
            }
        }
        if let Some(why) = &yaml.why {
            if !why.trim().is_empty() {
                out.push_str(&format!("- **Why:** {}\n", why.trim()));
            }
        }
        if let Some(rules) = &yaml.rules {
            if !rules.is_empty() {
                out.push_str(&format!("- **Rules:** {}\n", rules.join(", ")));
            }
        }
        if let Some(use_cases) = &yaml.use_cases {
            if !use_cases.is_empty() {
                out.push_str(&format!("- **Use cases:** {}\n", use_cases.join(", ")));
            }
        }
        if let Some(related) = &yaml.related {
            if !related.is_empty() {
                out.push_str(&format!("- **Related:** {}\n", related.join(", ")));
            }
        }
        if !entry.capabilities.is_empty() {
            out.push_str(&format!("- **Capabilities:** {}\n", entry.capabilities.join(", ")));
        }
        if let Some(required) = &yaml.required_parts {
            if !required.is_empty() {
                out.push_str(&format!("- **Required parts:** {}\n", required.join(", ")));
            }
        }
        if let Some(optional) = &yaml.optional_parts {
            if !optional.is_empty() {
                out.push_str(&format!("- **Optional parts:** {}\n", optional.join(", ")));
            }
        }
        if let Some(states) = &yaml.states {
            if !states.is_empty() {
                out.push_str(&format!("- **States:** {}\n", states.join(", ")));
            }
        }
        if let Some(bt) = &yaml.boundary_type {
            if !bt.is_empty() {
                out.push_str(&format!("- **Boundary type:** {}\n", bt));
            }
        }
        // canonical import — gerado automaticamente
        let id_snake = entry.id.replace('-', "_");
        out.push_str(&format!("- **Import:** `use canonrs::ui::{}::{};`\n",
            id_snake,
            to_pascal_case(&entry.id)
        ));

        // before/after
        if let (Some(before), Some(after)) = (&yaml.before, &yaml.after) {
            if !before.is_empty() && !after.is_empty() {
                out.push_str("\n### Migration\n\n");
                out.push_str(&format!("```rust\n{}\n```\n\n", before.trim()));
                out.push_str(&format!("```rust\n{}\n```\n\n", after.trim()));
            }
        }

        out.push_str("\n---\n\n");
    }

    out
}

fn build_llm_blocks(blocks: &[BlockInfo], blocks_dir: &Path) -> String {
    let mut out = String::new();
    out.push_str("# CanonRS — Blocks\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let block_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "block").collect();

    for block in &block_items {
        let id_snake = block.id.replace('-', "_");
        let block_dir = blocks_dir.join(&id_snake);
        let yaml = read_builder_yaml(&block_dir);
        let src_file = block_dir.join(format!("{}_block.rs", id_snake));
        let src = match fs::read_to_string(&src_file) { Ok(c) => c, Err(_) => continue };

        let description = yaml.description.clone()
            .or_else(|| extract_canon_field(&src, "canon-description"))
            .unwrap_or_else(|| format!("{} block", block.id));
        let label = yaml.label.clone()
            .or_else(|| block.label.clone())
            .unwrap_or_else(|| block.id.clone());

        out.push_str(&format!("## `{}`\n\n", block.id));
        out.push_str(&format!("- **Label:** {}\n", label));
        out.push_str(&format!("- **Description:** {}\n", description));
        out.push_str(&format!("- **Category:** {}\n", block.category));

        if let Some(intent) = &yaml.intent {
            if !intent.is_empty() {
                out.push_str(&format!("- **Intent:** {}\n", intent));
            }
        }
        if let Some(keywords) = &yaml.keywords {
            if !keywords.is_empty() {
                out.push_str(&format!("- **Keywords:** {}\n", keywords));
            }
        }
        if let Some(pain) = &yaml.pain {
            if !pain.is_empty() {
                out.push_str(&format!("- **Pain:** {}\n", pain));
            }
        }
        if let Some(promise) = &yaml.promise {
            if !promise.is_empty() {
                out.push_str(&format!("- **Promise:** {}\n", promise));
            }
        }
        if let Some(why) = &yaml.why {
            if !why.trim().is_empty() {
                out.push_str(&format!("- **Why:** {}\n", why.trim()));
            }
        }
        if let Some(rules) = &yaml.rules {
            if !rules.is_empty() {
                out.push_str(&format!("- **Rules:** {}\n", rules.join(", ")));
            }
        }
        if let Some(related) = &yaml.related {
            if !related.is_empty() {
                out.push_str(&format!("- **Related:** {}\n", related.join(", ")));
            }
        }

        if !block.regions.is_empty() {
            out.push_str(&format!("- **Regions:** {}\n", block.regions.join(", ")));
        }

        let slot_accepts = parse_slot_accepts(&src);
        if !slot_accepts.is_empty() {
            out.push_str("\n### Region Rules\n\n");
            out.push_str("| Region | Accepts |\n");
            out.push_str("|--------|---------|\n");
            for (region, accepts) in &slot_accepts {
                out.push_str(&format!("| `{}` | {} |\n", region, accepts));
            }
        }

        let props = parse_block_props(&src);
        if !props.is_empty() {
            out.push_str("\n### Props\n\n");
            out.push_str("| Prop | Type | Default | Scope |\n");
            out.push_str("|------|------|---------|-------|\n");
            for prop in &props {
                let default = prop.default.as_deref().unwrap_or("-");
                out.push_str(&format!("| `{}` | {} | `{}` | {} |\n", prop.key, prop.field, default, prop.scope));
            }
        }

        let presets = parse_block_presets(&src);
        if !presets.is_empty() {
            out.push_str("\n### Presets\n\n");
            for preset in &presets {
                let props_str: Vec<String> = preset.props.iter().map(|(k, v)| format!("`{}={}`", k, v)).collect();
                out.push_str(&format!("- **{}:** {}\n", preset.label, props_str.join(", ")));
            }
        }

        if let (Some(before), Some(after)) = (&yaml.before, &yaml.after) {
            if !before.is_empty() && !after.is_empty() {
                out.push_str("\n### Migration\n\n");
                out.push_str(&format!("```rust\n{}\n```\n\n", before.trim()));
                out.push_str(&format!("```rust\n{}\n```\n\n", after.trim()));
            }
        }

        out.push_str("\n---\n\n");
    }

    out
}

fn build_llm_layouts(blocks: &[BlockInfo], layouts_dir: &Path) -> String {
    let mut out = String::new();
    out.push_str("# CanonRS — Layouts\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let layout_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "layout").collect();

    for layout in &layout_items {
        let id_snake = layout.id.replace('-', "_");
        let layout_dir = layouts_dir.join(&id_snake);
        let yaml = read_builder_yaml(&layout_dir);

        let files = [
            layout_dir.join(format!("{}_layout.rs", id_snake)),
            layouts_dir.join(id_snake.replace("_layout", "")).join(format!("{}_layout.rs", id_snake.replace("_layout", ""))),
        ];
        let src = files.iter().find_map(|p| fs::read_to_string(p).ok()).unwrap_or_default();

        let description = yaml.description.clone()
            .or_else(|| extract_canon_field(&src, "canon-description"))
            .unwrap_or_else(|| format!("{} layout", layout.id));
        let label = yaml.label.clone()
            .or_else(|| layout.label.clone())
            .unwrap_or_else(|| layout.id.clone());

        out.push_str(&format!("## `{}`\n\n", layout.id));
        out.push_str(&format!("- **Label:** {}\n", label));
        out.push_str(&format!("- **Description:** {}\n", description));
        out.push_str(&format!("- **Category:** {}\n", layout.category));

        if let Some(intent) = &yaml.intent {
            if !intent.is_empty() {
                out.push_str(&format!("- **Intent:** {}\n", intent));
            }
        }
        if let Some(keywords) = &yaml.keywords {
            if !keywords.is_empty() {
                out.push_str(&format!("- **Keywords:** {}\n", keywords));
            }
        }
        if let Some(pain) = &yaml.pain {
            if !pain.is_empty() {
                out.push_str(&format!("- **Pain:** {}\n", pain));
            }
        }
        if let Some(promise) = &yaml.promise {
            if !promise.is_empty() {
                out.push_str(&format!("- **Promise:** {}\n", promise));
            }
        }
        if let Some(why) = &yaml.why {
            if !why.trim().is_empty() {
                out.push_str(&format!("- **Why:** {}\n", why.trim()));
            }
        }
        if let Some(rules) = &yaml.rules {
            if !rules.is_empty() {
                out.push_str(&format!("- **Rules:** {}\n", rules.join(", ")));
            }
        }
        if let Some(use_when) = &yaml.use_cases {
            if !use_when.is_empty() {
                out.push_str(&format!("- **Use when:** {}\n", use_when.join(", ")));
            }
        }

        if !layout.regions.is_empty() {
            out.push_str(&format!("- **Regions:** {}\n", layout.regions.join(", ")));
        }

        let slots = parse_slot_descriptions(&src);
        if !slots.is_empty() {
            out.push_str("\n### Slots\n\n");
            out.push_str("| Slot | Description |\n");
            out.push_str("|------|-------------|\n");
            for (id, desc) in &slots {
                out.push_str(&format!("| `{}` | {} |\n", id, desc));
            }
        }

        let slot_accepts = parse_slot_accepts(&src);
        if !slot_accepts.is_empty() {
            out.push_str("\n### Slot Rules\n\n");
            out.push_str("| Slot | Accepts |\n");
            out.push_str("|------|---------|\n");
            for (slot, accepts) in &slot_accepts {
                out.push_str(&format!("| `{}` | {} |\n", slot, accepts));
            }
        }

        if let (Some(before), Some(after)) = (&yaml.before, &yaml.after) {
            if !before.is_empty() && !after.is_empty() {
                out.push_str("\n### Migration\n\n");
                out.push_str(&format!("```rust\n{}\n```\n\n", before.trim()));
                out.push_str(&format!("```rust\n{}\n```\n\n", after.trim()));
            }
        }

        out.push_str("\n---\n\n");
    }

    out
}

fn build_llm_layout_primitives() -> String {
    // Path relativo ao canonrs-core
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let primitives_layout_dir = std::path::Path::new(&manifest).join("src/primitives/layout");

    let mut out = String::new();
    out.push_str("# CanonRS — Layout Primitives\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let primitives = [
        ("stack",     "StackPrimitive",     "Vertical or horizontal linear flex layout — use for most layouts"),
        ("flex",      "FlexPrimitive",      "Full-control flex — use when Stack is not enough"),
        ("grid",      "GridPrimitive",      "CSS grid — use for multi-column content grids"),
        ("container", "ContainerPrimitive", "Max-width centered wrapper — use at page level"),
        ("center",    "CenterPrimitive",    "Centers content — use for single centered elements"),
        ("spacer",    "SpacerPrimitive",    "Flex expander — use to push content apart"),
    ];

    for (name, component, use_when) in &primitives {
        let yaml = read_builder_yaml(&primitives_layout_dir.join(name));
        let src_path = primitives_layout_dir.join(format!("{}.rs", name));
        let _src = fs::read_to_string(&src_path).unwrap_or_default();

        let description = yaml.description.clone().unwrap_or_else(|| use_when.to_string());
        let label = yaml.label.clone().unwrap_or_else(|| component.to_string());

        out.push_str(&format!("## `{}`\n\n", name));
        out.push_str(&format!("- **Component:** `{}`\n", component));
        out.push_str(&format!("- **Label:** {}\n", label));
        out.push_str(&format!("- **Description:** {}\n", description));
        out.push_str(&format!("- **Use when:** {}\n", use_when));

        if let Some(tags) = &yaml.tags {
            if !tags.is_empty() {
                out.push_str(&format!("- **Tags:** {}\n", tags.join(", ")));
            }
        }

        out.push_str("\n### Decision Guide\n\n");
        out.push_str(match *name {
            "stack"     => "- Default choice for linear layouts\n- Vertical by default\n- Use `gap` for spacing\n- No justify — use Flex if needed\n",
            "flex"      => "- Escape hatch when Stack is not enough\n- Full control: direction, justify, align, gap, wrap\n- Use sparingly — prefer Stack\n",
            "grid"      => "- Use for multi-column content (cards, stats, thumbnails)\n- Cols: 1,2,3,4,6,12,auto\n- Not for page layout — use PageLayout\n",
            "container" => "- Always wrap page content with Container\n- Size lg by default\n- Full for edge-to-edge\n",
            "center"    => "- Use for single centered element (login, empty state, modal)\n- Both by default (horizontal + vertical)\n",
            "spacer"    => "- Only inside Flex or Stack horizontal\n- Pushes siblings apart\n- No children\n",
            _           => "",
        });

        out.push_str("\n---\n\n");
    }

    out
}

fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect()
}
