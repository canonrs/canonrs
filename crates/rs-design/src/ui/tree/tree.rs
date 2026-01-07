use leptos::prelude::*;
use super::{TreeNode, TreeNodeItem};

/// Tree - Hierarchical tree view
/// 
/// **Type:** Stateful Component (Type 2)
/// **Pattern:** Recursive rendering with expand/collapse
/// **Tokens:** 100% Canonical
#[component]
pub fn Tree(
    /// Root nodes
    #[prop(into)]
    nodes: Signal<Vec<TreeNode>>,
    
    /// Currently selected node ID
    #[prop(into)]
    selected_id: Signal<Option<String>>,
    
    /// Callback when node is selected
    #[prop(into)]
    on_select: Callback<String>,
    
    /// Callback when node expand state changes
    #[prop(into)]
    on_toggle: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="w-full bg-background border-r border-border">
            {move || {
                nodes.get().into_iter().map(|node| {
                    render_node_recursive(
                        node,
                        0,
                        selected_id,
                        on_select,
                        on_toggle
                    )
                }).collect_view()
            }}
        </div>
    }
}

/// Recursively render node and its children
fn render_node_recursive(
    node: TreeNode,
    depth: usize,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let children = node.children.clone();
    let is_expanded = node.expanded;
    
    view! {
        <div>
            <TreeNodeItem
                node=node
                depth=depth
                selected=Signal::derive(move || {
                    selected_id.get().as_ref() == Some(&node_id)
                })
                on_select=on_select
                on_toggle=on_toggle
            />
            
            // Render children if expanded
            {if is_expanded && !children.is_empty() {
                children.into_iter().map(|child| {
                    render_node_recursive(
                        child,
                        depth + 1,
                        selected_id,
                        on_select,
                        on_toggle
                    )
                }).collect_view().into_any()
            } else {
                view! { <></> }.into_any()
            }}
        </div>
    }
}
