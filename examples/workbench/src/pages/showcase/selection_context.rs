use leptos::prelude::*;

/// SelectionContext - Tracks currently selected tree node
/// 
/// **Pattern:** Context Provider
/// **Purpose:** Share selection state across Tree and Command Palette
#[derive(Clone, Debug)]
pub struct SelectionContext {
    /// Currently selected node ID
    pub selected_id: Option<String>,
    
    /// Selected node type (e.g., "workflow", "step", "user")
    pub node_type: Option<String>,
    
    /// Selected node label
    pub label: Option<String>,
    
    /// Optional metadata
    pub metadata: Option<String>,
}

impl SelectionContext {
    pub fn new() -> Self {
        Self {
            selected_id: None,
            node_type: None,
            label: None,
            metadata: None,
        }
    }
    
    pub fn with_selection(
        selected_id: String,
        node_type: String,
        label: String,
        metadata: Option<String>,
    ) -> Self {
        Self {
            selected_id: Some(selected_id),
            node_type: Some(node_type),
            label: Some(label),
            metadata,
        }
    }
    
    pub fn is_type(&self, type_name: &str) -> bool {
        self.node_type.as_deref() == Some(type_name)
    }
    
    pub fn has_selection(&self) -> bool {
        self.selected_id.is_some()
    }
}

impl Default for SelectionContext {
    fn default() -> Self {
        Self::new()
    }
}
