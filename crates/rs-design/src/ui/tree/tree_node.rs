/// TreeNode - Pure hierarchical data structure
/// 
/// **Type:** Pure data (no UI, no logic)
/// **Purpose:** Represents hierarchical domain structure
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    /// Unique identifier
    pub id: String,
    
    /// Display label
    pub label: String,
    
    /// Node type (e.g., "workflow", "step", "user", "role")
    pub node_type: String,
    
    /// Optional icon
    pub icon: Option<String>,
    
    /// Child nodes
    pub children: Vec<TreeNode>,
    
    /// Whether node is expanded (UI state)
    pub expanded: bool,
    
    /// Optional metadata (domain-specific data)
    pub metadata: Option<String>,
}

impl TreeNode {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        node_type: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            node_type: node_type.into(),
            icon: None,
            children: Vec::new(),
            expanded: false,
            metadata: None,
        }
    }
    
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    
    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
    
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }
    
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
    
    /// Find node by ID (recursive)
    pub fn find(&self, id: &str) -> Option<&TreeNode> {
        if self.id == id {
            return Some(self);
        }
        
        for child in &self.children {
            if let Some(found) = child.find(id) {
                return Some(found);
            }
        }
        
        None
    }
    
    /// Find node by ID (mutable)
    pub fn find_mut(&mut self, id: &str) -> Option<&mut TreeNode> {
        if self.id == id {
            return Some(self);
        }
        
        for child in &mut self.children {
            if let Some(found) = child.find_mut(id) {
                return Some(found);
            }
        }
        
        None
    }
    
    /// Get all node IDs (flattened)
    pub fn all_ids(&self) -> Vec<String> {
        let mut ids = vec![self.id.clone()];
        for child in &self.children {
            ids.extend(child.all_ids());
        }
        ids
    }
    
    /// Check if node has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
    
    /// Get depth level
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            0
        } else {
            1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0)
        }
    }
}
