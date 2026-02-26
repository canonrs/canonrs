use super::super::domain::constraints::{BlockDef, ComponentDef, NodeCategory};
use super::super::types::ActiveLayout;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CanvasMode { Builder, Preview, Wireframe }

#[derive(Clone, Debug)]
pub enum DragPayload {
    Block(BlockDef),
    Component(ComponentDef),
}

impl DragPayload {
    pub fn category(&self) -> NodeCategory {
        match self { Self::Block(b) => b.category, Self::Component(c) => c.category }
    }
    pub fn label(&self) -> &str {
        match self { Self::Block(b) => b.label, Self::Component(c) => c.label }
    }
    pub fn as_block(&self) -> Option<&BlockDef> {
        match self { Self::Block(b) => Some(b), _ => None }
    }
    pub fn as_component(&self) -> Option<&ComponentDef> {
        match self { Self::Component(c) => Some(c), _ => None }
    }
}

#[derive(Clone, Debug)]
pub struct DragContext {
    pub node_id: Option<Uuid>,
    pub block_def: Option<BlockDef>,
    pub component_def: Option<ComponentDef>,
    pub layout_def: Option<ActiveLayout>,
}

impl DragContext {
    pub fn empty() -> Self { Self { node_id: None, block_def: None, component_def: None, layout_def: None } }
    pub fn is_dragging(&self) -> bool {
        self.block_def.is_some() || self.component_def.is_some() || self.node_id.is_some() || self.layout_def.is_some()
    }
    pub fn payload(&self) -> Option<DragPayload> {
        if let Some(b) = &self.block_def { return Some(DragPayload::Block(b.clone())); }
        if let Some(c) = &self.component_def { return Some(DragPayload::Component(c.clone())); }
        None
    }
}
