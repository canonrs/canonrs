//! Navigation Context - Shared state for doc navigation
//! Provides heading hierarchy, scroll progress, active branch

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct NavigationState {
    pub current_heading_id: Option<String>,
    pub heading_hierarchy: HeadingHierarchy,
    pub scroll_progress: f32, // 0.0 - 1.0
    pub active_branch: Vec<String>, // IDs from root to current
    pub depth_current: u8,
}

#[derive(Clone, Debug, Default)]
pub struct HeadingHierarchy {
    pub headings: Vec<HeadingNode>,
    pub id_to_index: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub struct HeadingNode {
    pub id: String,
    pub text: String,
    pub level: u8,
    pub parent_id: Option<String>,
    pub children_ids: Vec<String>,
}

impl NavigationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_active_heading(&mut self, heading_id: String) {
        self.current_heading_id = Some(heading_id.clone());
        self.active_branch = self.heading_hierarchy.get_ancestor_chain(&heading_id);
        self.depth_current = self.heading_hierarchy
            .get_level(&heading_id)
            .unwrap_or(1);
    }

    pub fn update_scroll_progress(&mut self, progress: f32) {
        self.scroll_progress = progress.clamp(0.0, 1.0);
    }
}

impl HeadingHierarchy {
    pub fn from_toc_items(items: &[crate::TocItem]) -> Self {
        let mut hierarchy = HeadingHierarchy::default();
        let mut stack: Vec<(usize, u8)> = Vec::new(); // (index, level)

        for item in items {
            let index = hierarchy.headings.len();
            let level = item.level;
            
            // Find parent
            while stack.last().map(|(_, l)| *l >= level).unwrap_or(false) {
                stack.pop();
            }

            let parent_id = stack.last().map(|(idx, _)| {
                hierarchy.headings[*idx].id.clone()
            });

            let node = HeadingNode {
                id: item.id.clone(),
                text: item.text.clone(),
                level: item.level,
                parent_id: parent_id.clone(),
                children_ids: Vec::new(),
            };

            // Update parent's children
            if let Some((parent_idx, _)) = stack.last() {
                hierarchy.headings[*parent_idx].children_ids.push(item.id.clone());
            }

            hierarchy.id_to_index.insert(item.id.clone(), index);
            hierarchy.headings.push(node);
            stack.push((index, level));
        }

        hierarchy
    }

    pub fn get_ancestor_chain(&self, id: &str) -> Vec<String> {
        let mut chain = Vec::new();
        let mut current_id = Some(id.to_string());

        while let Some(id) = current_id {
            chain.push(id.clone());
            current_id = self.id_to_index
                .get(&id)
                .and_then(|idx| self.headings[*idx].parent_id.clone());
        }

        chain.reverse();
        chain
    }

    pub fn get_level(&self, id: &str) -> Option<u8> {
        self.id_to_index
            .get(id)
            .map(|idx| self.headings[*idx].level)
    }

    pub fn get_breadcrumb(&self, id: &str) -> Vec<(String, String)> {
        self.get_ancestor_chain(id)
            .into_iter()
            .filter_map(|id| {
                self.id_to_index.get(&id).map(|idx| {
                    let node = &self.headings[*idx];
                    (node.id.clone(), node.text.clone())
                })
            })
            .collect()
    }
}
