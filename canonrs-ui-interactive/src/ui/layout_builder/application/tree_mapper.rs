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
        }
    }
}

/// flatten_tree permanece na UI — depende de Node (builder model)
pub fn flatten_tree(nodes: &[CanonNode]) -> Vec<Node> {
    let mut flat = vec![];
    for (i, node) in nodes.iter().enumerate() {
        flatten_node(node, None, i, &mut flat);
    }
    flat
}

fn flatten_node(canon: &CanonNode, parent_id: Option<Uuid>, _index: usize, flat: &mut Vec<Node>) {
    let kind = match &canon.block {
        CanonBlockType::Slot { name } => NodeKind::Slot { name: name.clone() },
        block_type => NodeKind::Block {
            def: BlockDef {
                id: block_type.to_id(), label: block_type.to_id(), icon: "▭",
                category: NodeCategory::Content,
                is_container: !canon.children.is_empty(),
                regions: &[],
            }
        }
    };
    flat.push(Node { id: canon.id, kind, parent_id });
    for (i, child) in canon.children.iter().enumerate() {
        flatten_node(child, Some(canon.id), i, flat);
    }
}


pub fn build_tree(flat: &[crate::ui::layout_builder::domain::node::Node]) -> Vec<rs_canonrs::domain::CanonNode> {
    build_tree_core(flat)
}
