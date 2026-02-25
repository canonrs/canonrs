use uuid::Uuid;
use super::constraints::{BlockDef, BlockRegion, ComponentDef, NodeCategory, allowed_children, slot_accepts};
use super::blocks::{get_block, BLOCK_REGISTRY};
use super::layout::ActiveLayout;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextVariant { Heading1, Heading2, Heading3, Paragraph, Label, Caption }

impl TextVariant {
    pub fn tag(&self) -> &'static str {
        match self { Self::Heading1=>"h1", Self::Heading2=>"h2", Self::Heading3=>"h3",
                     Self::Paragraph=>"p", Self::Label=>"label", Self::Caption=>"span" }
    }
    pub fn label(&self) -> &'static str {
        match self { Self::Heading1=>"Heading 1", Self::Heading2=>"Heading 2",
                     Self::Heading3=>"Heading 3", Self::Paragraph=>"Paragraph",
                     Self::Label=>"Label", Self::Caption=>"Caption" }
    }
    pub fn placeholder(&self) -> &'static str {
        match self { Self::Heading1=>"Page title", Self::Heading2=>"Section title",
                     Self::Heading3=>"Subsection title", Self::Paragraph=>"Write something...",
                     Self::Label=>"Label text", Self::Caption=>"Caption text" }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Slot { name: String },
    Block { def: BlockDef },
    Region { block_id: String, region_id: String, label: String },
    Component { def: ComponentDef },
    Text { content: String, variant: TextVariant },
}

/// Node sem index — posição calculada dinamicamente via posição no vetor de filhos
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: Uuid,
    pub kind: NodeKind,
    pub parent_id: Option<Uuid>,
}

impl Node {
    pub fn slot(name: &str) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Slot { name: name.to_string() }, parent_id: None }
    }
    pub fn block(def: BlockDef, parent_id: Uuid) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Block { def }, parent_id: Some(parent_id) }
    }
    pub fn region(block_id: &str, region: &BlockRegion, parent_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: NodeKind::Region { block_id: block_id.to_string(), region_id: region.id.to_string(), label: region.label.to_string() },
            parent_id: Some(parent_id),
        }
    }
    pub fn text(variant: TextVariant, content: String, parent_id: Uuid) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Text { content, variant }, parent_id: Some(parent_id) }
    }
    pub fn component(def: ComponentDef, parent_id: Uuid) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Component { def }, parent_id: Some(parent_id) }
    }
    pub fn is_container(&self) -> bool {
        match &self.kind {
            NodeKind::Slot { .. } | NodeKind::Region { .. } => true,
            NodeKind::Block { def } => def.is_container && def.regions.is_empty(),
            _ => false,
        }
    }
    pub fn label(&self) -> &str {
        match &self.kind {
            NodeKind::Slot { name } => name,
            NodeKind::Block { def } => def.label,
            NodeKind::Region { label, .. } => label,
            NodeKind::Component { def } => def.label,
            NodeKind::Text { variant, .. } => variant.label(),
        }
    }
    pub fn accepts(&self, child: &BlockDef) -> bool {
        match &self.kind {
            NodeKind::Slot { .. } => slot_accepts(child.category),
            NodeKind::Block { def } => def.can_accept(child.category),
            NodeKind::Region { block_id, region_id, .. } => {
                get_block(block_id)
                    .and_then(|b| b.regions.iter().find(|r| r.id == *region_id))
                    .map(|r| r.accepts_block(child.id, child.category))
                    .unwrap_or(false)
            },
            _ => false,
        }
    }
    pub fn accepts_component(&self, child: &ComponentDef) -> bool {
        match &self.kind {
            NodeKind::Slot { .. } => slot_accepts(child.category),
            NodeKind::Block { def } => def.can_accept(child.category),
            NodeKind::Region { block_id, region_id, .. } => {
                get_block(block_id)
                    .and_then(|b| b.regions.iter().find(|r| r.id == *region_id))
                    .map(|r| r.accepts_block(child.id, child.category))
                    .unwrap_or(false)
            },
            _ => false,
        }
    }
}

// ─── Tree Operations ──────────────────────────────────────────────────────────

/// Retorna filhos ordenados por posição no vetor (sem index)
pub fn children_of(tree: &[Node], parent_id: Uuid) -> Vec<Node> {
    tree.iter()
        .filter(|n| n.parent_id == Some(parent_id))
        .cloned()
        .collect()
}

/// Insere nó na posição idx entre os filhos do mesmo pai
pub fn insert_node(tree: &mut Vec<Node>, node: Node, idx: usize) {
    let parent_id = node.parent_id;
    // Encontra os índices no vetor dos filhos do mesmo pai
    let sibling_positions: Vec<usize> = tree.iter().enumerate()
        .filter(|(_, n)| n.parent_id == parent_id)
        .map(|(i, _)| i)
        .collect();
    // Insere na posição correta
    let insert_at = if idx >= sibling_positions.len() {
        tree.len() // appenda ao final
    } else {
        sibling_positions[idx]
    };
    tree.insert(insert_at, node);
}

pub fn remove_node(tree: &mut Vec<Node>, id: Uuid) {
    use std::collections::HashSet;
    let mut to_remove: HashSet<Uuid> = HashSet::new();
    to_remove.insert(id);
    loop {
        let before = to_remove.len();
        let children: Vec<Uuid> = tree.iter()
            .filter(|n| n.parent_id.map(|p| to_remove.contains(&p)).unwrap_or(false))
            .map(|n| n.id)
            .collect();
        to_remove.extend(children);
        if to_remove.len() == before { break; }
    }
    tree.retain(|n| to_remove.contains(&n.id) == false);
}

pub fn move_node(tree: &mut Vec<Node>, node_id: Uuid, new_parent_id: Uuid, new_idx: usize) {
    let node = match tree.iter().find(|n| n.id == node_id).cloned() {
        Some(n) => n, None => return,
    };
    remove_node(tree, node_id);
    let new_node = Node { id: node_id, kind: node.kind, parent_id: Some(new_parent_id) };
    insert_node(tree, new_node, new_idx);
}

pub fn init_slots(layout: &ActiveLayout) -> Vec<Node> {
    layout.slots().into_iter().map(|name| Node::slot(name)).collect()
}

