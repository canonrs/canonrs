//! Generator: schema.json

use std::fs;
use std::collections::HashMap;
use super::types::*;

pub(crate) fn generate_schema_json(
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
