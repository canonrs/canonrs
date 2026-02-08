use serde::{Serialize, Deserialize};

/// Unique identifier for draggable items
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DragItemId(pub String);

impl DragItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Unique identifier for drop targets
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DropTargetId(pub String);

impl DropTargetId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Current drag state
#[derive(Clone, Debug, PartialEq)]
pub enum DragState {
    /// Nothing being dragged
    Idle,
    
    /// Item is being dragged
    Dragging {
        /// ID of item being dragged
        item_id: DragItemId,
        
        /// Optional data payload (serialized)
        data: Option<String>,
        
        /// Current drop target being hovered (if any)
        hover_target: Option<DropTargetId>,
    },
}

impl Default for DragState {
    fn default() -> Self {
        Self::Idle
    }
}

impl DragState {
    pub fn is_dragging(&self) -> bool {
        matches!(self, DragState::Dragging { .. })
    }
    
    pub fn is_hovering(&self, target_id: &DropTargetId) -> bool {
        match self {
            DragState::Dragging { hover_target: Some(id), .. } => id == target_id,
            _ => false,
        }
    }
    
    pub fn dragging_item(&self) -> Option<&DragItemId> {
        match self {
            DragState::Dragging { item_id, .. } => Some(item_id),
            _ => None,
        }
    }
}

/// Event emitted when drag starts
#[derive(Clone, Debug)]
pub struct DragStartEvent {
    pub item_id: DragItemId,
    pub data: Option<String>,
}

/// Event emitted when item is dropped
#[derive(Clone, Debug)]
pub struct DropEvent {
    pub item_id: DragItemId,
    pub target_id: DropTargetId,
    pub data: Option<String>,
}
