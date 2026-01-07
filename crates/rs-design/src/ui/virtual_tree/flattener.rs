use crate::ui::tree::TreeNode;
use super::types::VirtualTreeNode;

/// Flatten hierarchical TreeNode into flat list for virtualization
pub fn flatten_tree(nodes: &[TreeNode]) -> Vec<VirtualTreeNode> {
    let mut result = Vec::new();
    flatten_recursive(nodes, 0, None, &mut result);
    result
}

fn flatten_recursive(
    nodes: &[TreeNode],
    depth: usize,
    parent_id: Option<String>,
    result: &mut Vec<VirtualTreeNode>,
) {
    for node in nodes {
        let virtual_node = VirtualTreeNode::new(
            node.id.clone(),
            node.label.clone(),
            node.node_type.clone(),
            depth,
        )
        .with_children(!node.children.is_empty())
        .with_expanded(node.expanded);

        let virtual_node = if let Some(icon) = &node.icon {
            virtual_node.with_icon(icon.clone())
        } else {
            virtual_node
        };

        let virtual_node = if let Some(meta) = &node.metadata {
            virtual_node.with_metadata(meta.clone())
        } else {
            virtual_node
        };

        let virtual_node = if let Some(pid) = &parent_id {
            virtual_node.with_parent(pid.clone())
        } else {
            virtual_node
        };

        result.push(virtual_node);

        // Only flatten children if node is expanded
        if node.expanded && !node.children.is_empty() {
            flatten_recursive(
                &node.children,
                depth + 1,
                Some(node.id.clone()),
                result,
            );
        }
    }
}
