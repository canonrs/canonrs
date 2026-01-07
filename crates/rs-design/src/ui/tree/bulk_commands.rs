use crate::commands::Command;
use crate::ui::tree::TreeNode;
use leptos::prelude::*;

/// Bulk Delete Command - removes multiple nodes
#[derive(Debug, Clone)]
pub struct BulkDeleteNodesCommand {
    pub nodes_signal: RwSignal<Vec<TreeNode>>,
    pub deleted_nodes: Vec<(usize, TreeNode)>, // (original_index, node)
}

impl Command for BulkDeleteNodesCommand {
    fn execute(&self) {
        self.nodes_signal.update(|nodes: &mut Vec<TreeNode>| {
            for (_, node) in &self.deleted_nodes {
                nodes.retain(|n| n.id != node.id);
            }
        });
    }
    
    fn undo(&self) {
        self.nodes_signal.update(|nodes: &mut Vec<TreeNode>| {
            for (index, node) in &self.deleted_nodes {
                if *index <= nodes.len() {
                    nodes.insert(*index, node.clone() as TreeNode);
                } else {
                    nodes.push(node.clone());
                }
            }
        });
    }
    
    fn description(&self) -> String {
        format!("Delete {} nodes", self.deleted_nodes.len())
    }
}

/// Bulk Move Command - moves multiple nodes
#[derive(Debug, Clone)]
pub struct BulkMoveNodesCommand {
    pub nodes_signal: RwSignal<Vec<TreeNode>>,
    pub moves: Vec<(String, String)>, // (node_id, new_parent_id)
    pub original_parents: Vec<(String, Option<String>)>, // (node_id, old_parent_id)
}

impl Command for BulkMoveNodesCommand {
    fn execute(&self) {
        self.nodes_signal.update(|nodes: &mut Vec<TreeNode>| {
            for (node_id, new_parent_id) in &self.moves {
                if let Some(node) = nodes.iter_mut().find(|n| &n.id == node_id) {
                }
            }
        });
    }
    
    fn undo(&self) {
        self.nodes_signal.update(|nodes: &mut Vec<TreeNode>| {
            for (node_id, old_parent_id) in &self.original_parents {
                if let Some(node) = nodes.iter_mut().find(|n| &n.id == node_id) {
                }
            }
        });
    }
    
    fn description(&self) -> String {
        format!("Move {} nodes", self.moves.len())
    }
}
