//! gen_llm.rs — Generates LLM context files
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use super::types::{SemanticEntry, BlockInfo};
use super::parsers::{parse_block_props, parse_block_presets, parse_slot_descriptions, parse_slot_accepts, extract_canon_field};

pub(crate) fn generate_llm_context(
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
    out.push_str("# CanonRS — UI Components\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let mut entries: Vec<&SemanticEntry> = semantic.values().collect();
    entries.sort_by(|a, b| a.id.cmp(&b.id));

    for entry in &entries {
        out.push_str(&format!("## `{}`\n\n", entry.id));
        out.push_str(&format!("- **Label:** {}\n", entry.label));
        out.push_str(&format!("- **Family:** {}\n", entry.family));
        out.push_str(&format!("- **Category:** {}\n", entry.catalog_category));
        out.push_str(&format!("- **Intent:** {}\n", entry.intent));
        out.push_str(&format!("- **Description:** {}\n", entry.description));
        out.push_str(&format!("- **Composable:** {}\n", if entry.composable { "yes" } else { "no" }));

        if !entry.capabilities.is_empty() {
            out.push_str(&format!("- **Capabilities:** {}\n", entry.capabilities.join(", ")));
        }
        if !entry.required_parts.is_empty() {
            out.push_str(&format!("- **Required parts:** {}\n", entry.required_parts.join(", ")));
        }
        if !entry.optional_parts.is_empty() {
            out.push_str(&format!("- **Optional parts:** {}\n", entry.optional_parts.join(", ")));
        }
        if !entry.catalog_tags.is_empty() {
            out.push_str(&format!("- **Tags:** {}\n", entry.catalog_tags.join(", ")));
        }

        // Usage context derived from family
        let usage = family_usage_context(&entry.family);
        if !usage.is_empty() {
            out.push_str(&format!("- **Use when:** {}\n", usage));
        }

        // Related components derived from category
        let related = category_related_components(&entry.catalog_category);
        if !related.is_empty() {
            out.push_str(&format!("- **Often used with:** {}\n", related));
        }

        out.push_str("\n---\n\n");
    }

    fs::write(out_dir.join("llm_components.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_components.md ({} components)", entries.len());
}

fn generate_llm_blocks(blocks: &[BlockInfo], blocks_dir: &Path, out_dir: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Blocks\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let block_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "block").collect();

    for block in &block_items {
        let id_snake = block.id.replace('-', "_");
        let src_file = blocks_dir.join(&id_snake).join(format!("{}_block.rs", id_snake));
        let src = match fs::read_to_string(&src_file) { Ok(c) => c, Err(_) => continue };

        let description = extract_canon_field(&src, "canon-description")
            .unwrap_or_else(|| format!("{} block", block.id));
        let label = block.label.clone().unwrap_or_else(|| block.id.clone());

        out.push_str(&format!("## `{}`\n\n", block.id));
        out.push_str(&format!("- **Label:** {}\n", label));
        out.push_str(&format!("- **Description:** {}\n", description));
        out.push_str(&format!("- **Category:** {}\n", block.category));
        out.push_str(&format!("- **Variant:** {}\n", block.variant));
        out.push_str(&format!("- **Container:** {}\n", if block.container { "yes" } else { "no" }));

        if !block.regions.is_empty() {
            out.push_str(&format!("- **Regions:** {}\n", block.regions.join(", ")));
        }

        // Slot accepts — region rules
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

        // Hierarchy hint
        let children_hint = block_children_hint(&block.category);
        if !children_hint.is_empty() {
            out.push_str(&format!("\n### Typical Children\n\n{}\n", children_hint));
        }

        out.push_str("\n---\n\n");
    }

    fs::write(out_dir.join("llm_blocks.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_blocks.md ({} blocks)", block_items.len());
}

fn generate_llm_layouts(blocks: &[BlockInfo], layouts_dir: &Path, out_dir: &Path) {
    let mut out = String::new();
    out.push_str("# CanonRS — Layouts\n\n");
    out.push_str("> AUTO-GENERATED — do not edit manually\n\n");
    out.push_str("---\n\n");

    let layout_items: Vec<&BlockInfo> = blocks.iter().filter(|b| b.kind == "layout").collect();

    for layout in &layout_items {
        let id_snake = layout.id.replace('-', "_");
        let files = [
            layouts_dir.join(&id_snake).join(format!("{}_layout.rs", id_snake)),
            layouts_dir.join(id_snake.replace("_layout", "")).join(format!("{}_layout.rs", id_snake.replace("_layout", ""))),
        ];
        let src = files.iter().find_map(|p| fs::read_to_string(p).ok()).unwrap_or_default();

        let description = extract_canon_field(&src, "canon-description")
            .unwrap_or_else(|| format!("{} layout", layout.id));
        let label = layout.label.clone().unwrap_or_else(|| layout.id.clone());
        let icon  = extract_canon_field(&src, "canon-icon").unwrap_or_default();

        out.push_str(&format!("## `{}`\n\n", layout.id));
        out.push_str(&format!("- **Label:** {} {}\n", label, icon));
        out.push_str(&format!("- **Description:** {}\n", description));
        out.push_str(&format!("- **Category:** {}\n", layout.category));

        if !layout.regions.is_empty() {
            out.push_str(&format!("- **Regions:** {}\n", layout.regions.join(", ")));
        }

        // Use when — derived from layout id
        let use_when = layout_use_when(&layout.id);
        if !use_when.is_empty() {
            out.push_str(&format!("- **Use when:** {}\n", use_when));
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

        out.push_str("\n---\n\n");
    }

    fs::write(out_dir.join("llm_layouts.md"), &out).unwrap();
    println!("cargo:warning=CanonRS LLM: llm_layouts.md ({} layouts)", layout_items.len());
}

fn family_usage_context(family: &str) -> &'static str {
    match family {
        "interactive"  => "triggering actions, user input, form submission",
        "feedback"     => "showing status, alerts, notifications, loading states",
        "overlay"      => "modal dialogs, drawers, popovers, tooltips",
        "navigation"   => "routing, menus, breadcrumbs, pagination",
        "layout"       => "structuring page content, containers, grids",
        "data_display" => "presenting data, lists, tables, charts, metrics",
        "input"        => "collecting user input, forms, search",
        "typography"   => "text display, labels, headings, links",
        "utility"      => "helper components, wrappers, decorators",
        _              => "",
    }
}

fn category_related_components(category: &str) -> &'static str {
    match category {
        "Action"     => "Button, IconButton, ButtonGroup, DropdownMenu",
        "Form"       => "Field, Label, Input, Select, Checkbox, Switch, Textarea",
        "Navigation" => "Breadcrumb, Pagination, Tabs, Sidebar, NavItem",
        "Feedback"   => "Toast, Alert, Banner, Spinner, Skeleton, Progress",
        "Overlay"    => "Dialog, Drawer, Sheet, Modal, Popover, Tooltip",
        "Display"    => "Badge, Avatar, Card, Stat, Icon, Chart",
        "Data"       => "DataTable, DataTableColumn, Pagination, EmptyTable",
        "Layout"     => "Card, Container, Grid, Stack, Columns",
        _            => "",
    }
}

fn block_children_hint(category: &str) -> &'static str {
    match category {
        "layout"  => "card, grid, stack, columns, container",
        "data"    => "button, input, select, badge, stat-card",
        "form"    => "input, select, checkbox, switch, textarea, radio-group",
        "overlay" => "button, form, markdown, list",
        "content" => "markdown, code-block, stat-card, empty-state",
        "page"    => "header, footer, toolbar, page-header",
        _         => "",
    }
}

fn layout_use_when(id: &str) -> &'static str {
    match id {
        "dashboard"    => "admin panels, data-heavy apps, internal tools with sidebar navigation",
        "marketing"    => "public-facing pages, landing pages, product sites",
        "fullscreen"   => "focused editors, canvas tools, immersive experiences",
        "split-view"   => "master-detail views, side-by-side comparisons, email clients",
        "wizard-layout"=> "multi-step forms, onboarding flows, guided setup",
        "section"      => "content sections within a page, feature blocks",
        "page-layout"  => "documentation pages, articles, content with optional sidebar",
        _              => "",
    }
}
