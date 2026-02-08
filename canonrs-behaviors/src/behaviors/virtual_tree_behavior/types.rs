/// VirtualTreeNode - Flat representation for virtualization
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualTreeNode {
    /// Unique ID
    pub id: String,
    
    /// Display label
    pub label: String,
    
    /// Node type (workflow, step, user, etc)
    pub node_type: String,
    
    /// Optional icon
    pub icon: Option<String>,
    
    /// Depth level (0 = root)
    pub depth: usize,
    
    /// Has children
    pub has_children: bool,
    
    /// Is expanded
    pub expanded: bool,
    
    /// Parent ID (for finding path)
    pub parent_id: Option<String>,
    
    /// Metadata
    pub metadata: Option<String>,
}

impl VirtualTreeNode {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        node_type: impl Into<String>,
        depth: usize,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            node_type: node_type.into(),
            icon: None,
            depth,
            has_children: false,
            expanded: false,
            parent_id: None,
            metadata: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_children(mut self, has_children: bool) -> Self {
        self.has_children = has_children;
        self
    }

    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn with_parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollState {
    pub scroll_top: f64,
}
