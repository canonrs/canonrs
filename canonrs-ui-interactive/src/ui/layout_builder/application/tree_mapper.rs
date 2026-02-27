use uuid::Uuid;
use rs_canonrs::domain::{CanonBlockType, CanonNode, CanonDocument};
use rs_canonrs::application::FlatNode;
use rs_canonrs::application::build_tree as build_tree_core;
use crate::ui::layout_builder::domain::node::{Node, NodeKind};
use crate::ui::layout_builder::domain::constraints::{BlockDef, NodeCategory};

impl FlatNode for Node {
    fn node_id(&self) -> Uuid { self.id }
    fn parent_id(&self) -> Option<Uuid> { self.parent_id }
    fn to_canon_block(&self) -> CanonBlockType {
        match &self.kind {
            NodeKind::Slot { name } => CanonBlockType::Slot { name: name.clone() },
            NodeKind::Block { def } => CanonBlockType::from_id(def.id).expect("Unknown block type — registry mismatch"),
            NodeKind::Region { region_id, .. } => CanonBlockType::Slot { name: region_id.to_string() },
            NodeKind::Component { def } => CanonBlockType::from_id(def.id).expect("Unknown block type — registry mismatch"),
            NodeKind::Text { variant, .. } => CanonBlockType::from_id(variant.tag()).expect("Unknown block type — registry mismatch"),
            NodeKind::Layout { .. } => CanonBlockType::Layout,
        }
    }
}

/// flatten_tree permanece na UI — depende de Node (builder model)
pub fn flatten_tree(nodes: &[CanonNode]) -> Vec<Node> {
    let mut flat = vec![];
    for (i, node) in nodes.iter().enumerate() {
        flatten_node(node, None, i, &mut flat);
    }
    resolve_regions(&mut flat);
    flat
}

fn flatten_node(canon: &CanonNode, parent_id: Option<Uuid>, _index: usize, flat: &mut Vec<Node>) {
    let kind = match &canon.block {
        CanonBlockType::Layout => NodeKind::Layout { id: "layout".to_string(), label: "Layout".to_string() },
        CanonBlockType::Slot { name } => NodeKind::Slot { name: name.clone() },
        block_type => {
            use crate::ui::layout_builder::domain::blocks::BLOCK_REGISTRY;
            let registry_def = BLOCK_REGISTRY.values().find(|b| b.id == block_type.to_id()).cloned();
            if let Some(def) = registry_def {
                NodeKind::Block { def }
            } else {
                NodeKind::Block {
                    def: BlockDef {
                        id: block_type.to_id(), label: block_type.to_id(), icon: "▭",
                        category: NodeCategory::Content,
                        is_container: !canon.children.is_empty(),
                        regions: &[],
                    }
                }
            }
        }
    };
    flat.push(Node { id: canon.id, kind, parent_id });
    for (i, child) in canon.children.iter().enumerate() {
        flatten_node(child, Some(canon.id), i, flat);
    }
}

/// Pós-processamento: converte Slot filhos de Block em Region
fn resolve_regions(flat: &mut Vec<Node>) {
    let parent_kinds: std::collections::HashMap<Uuid, String> = flat.iter()
        .filter_map(|n| match &n.kind {
            NodeKind::Block { def } => Some((n.id, def.id.to_string())),
            _ => None,
        })
        .collect();

    for node in flat.iter_mut() {
        if let NodeKind::Slot { name } = &node.kind {
            if let Some(parent_id) = node.parent_id {
                if let Some(block_id) = parent_kinds.get(&parent_id) {
                    node.kind = NodeKind::Region {
                        block_id: block_id.clone(),
                        region_id: name.clone(),
                        label: name.clone(),
                    };
                }
            }
        }
    }
}

pub fn build_tree(flat: &[crate::ui::layout_builder::domain::node::Node]) -> Vec<rs_canonrs::domain::CanonNode> {
    build_tree_core(flat)
}
