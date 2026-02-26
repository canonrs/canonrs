use leptos::prelude::Get;
use crate::ui::theme_engine::{ThemeState, ThemeTokens};
use crate::ui::layout_builder::domain::node::{Node, NodeKind};
use serde::Serialize;

pub fn export_css(theme: &ThemeState) -> String {
    let light = theme.light.get();
    let dark  = theme.dark.get();
    let radius = theme.radius.get();
    format!(
        ":root {{\n{}\n  --radius: {}rem;\n}}\n\n[data-theme=\"dark\"] {{\n{}\n  --radius: {}rem;\n}}",
        tokens_to_css_block(&light), radius,
        tokens_to_css_block(&dark),  radius,
    )
}

fn tokens_to_css_block(t: &ThemeTokens) -> String {
    t.to_css_vars()
        .iter()
        .map(|(k, v)| format!("  --{}: {};", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn export_json(theme: &ThemeState) -> String {
    serde_json::json!({
        "active": theme.active.get(),
        "radius": theme.radius.get(),
        "light":  theme.light.get(),
        "dark":   theme.dark.get(),
    }).to_string()
}

#[derive(Serialize)]
struct ExportNode {
    id: String,
    parent_id: Option<String>,
    kind: serde_json::Value,
}

pub fn export_builder(tree: &[Node], active_layout: &str) -> String {
    let nodes: Vec<ExportNode> = tree.iter().map(|n| {
        let kind = match &n.kind {
            NodeKind::Slot { name } => serde_json::json!({ "type": "slot", "name": name }),
            NodeKind::Block { def } => serde_json::json!({ "type": "block", "id": def.id }),
            NodeKind::Region { block_id, region_id, .. } => serde_json::json!({ "type": "region", "block_id": block_id, "region_id": region_id }),
            NodeKind::Component { def } => serde_json::json!({ "type": "component", "id": def.id }),
            NodeKind::Text { content, variant } => serde_json::json!({ "type": "text", "variant": format!("{:?}", variant), "content": content }),
            NodeKind::Layout { id, .. } => serde_json::json!({ "type": "layout", "id": id }),
        };
        ExportNode { id: n.id.to_string(), parent_id: n.parent_id.map(|p| p.to_string()), kind }
    }).collect();

    serde_json::json!({ "layout": active_layout, "nodes": nodes }).to_string()
}

use crate::ui::layout_builder::domain::blocks::{get_block, get_component};

pub fn import_builder(json: &str) -> Option<Vec<Node>> {
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let nodes_json = v["nodes"].as_array()?;
    let mut nodes: Vec<Node> = Vec::new();

    for n in nodes_json {
        let id: uuid::Uuid = n["id"].as_str()?.parse().ok()?;
        let parent_id: Option<uuid::Uuid> = n["parent_id"].as_str()
            .and_then(|s| s.parse().ok());
        let kind_obj = &n["kind"];
        let t = kind_obj["type"].as_str()?;

        let kind = match t {
            "slot" => {
                let name = kind_obj["name"].as_str()?.to_string();
                NodeKind::Slot { name }
            }
            "block" => {
                let block_id = kind_obj["id"].as_str()?;
                let def = get_block(block_id)?.clone();
                NodeKind::Block { def }
            }
            "region" => {
                let block_id = kind_obj["block_id"].as_str()?.to_string();
                let region_id = kind_obj["region_id"].as_str()?.to_string();
                let label = get_block(&block_id)
                    .and_then(|b| b.regions.iter().find(|r| r.id == region_id))
                    .map(|r| r.label.to_string())
                    .unwrap_or_default();
                NodeKind::Region { block_id, region_id, label }
            }
            "component" => {
                let comp_id = kind_obj["id"].as_str()?;
                let def = get_component(comp_id)?.clone();
                NodeKind::Component { def }
            }
            "text" => {
                use crate::ui::layout_builder::domain::node::TextVariant;
                let content = kind_obj["content"].as_str()?.to_string();
                let variant = match kind_obj["variant"].as_str()? {
                    "Heading1"  => TextVariant::Heading1,
                    "Heading2"  => TextVariant::Heading2,
                    "Heading3"  => TextVariant::Heading3,
                    "Paragraph" => TextVariant::Paragraph,
                    "Label"     => TextVariant::Label,
                    _           => TextVariant::Caption,
                };
                NodeKind::Text { content, variant }
            }
            "layout" => {
                let id = kind_obj["id"].as_str()?.to_string();
                let label = id.clone();
                NodeKind::Layout { id, label }
            }
            _ => return None,
        };

        nodes.push(Node { id, parent_id, kind });
    }

    Some(nodes)
}
