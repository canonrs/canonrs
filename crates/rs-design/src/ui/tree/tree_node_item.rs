use leptos::prelude::*;
use super::tree_node::TreeNode;

/// TreeNodeItem - Renders a single tree node
/// 
/// **Type:** Pure Component (Type 1)
/// **Tokens:** 100% Canonical
#[component]
pub fn TreeNodeItem(
    /// Node to render
    node: TreeNode,
    
    /// Current depth level (for indentation)
    depth: usize,
    
    /// Whether this node is selected
    #[prop(into)]
    selected: Signal<bool>,
    
    /// Callback when node is clicked
    #[prop(into)]
    on_select: Callback<String>,
    
    /// Callback when expand/collapse is toggled
    #[prop(into)]
    on_toggle: Callback<String>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_for_toggle = node.id.clone();
    let has_children = node.has_children();
    let is_expanded = node.expanded;
    
    view! {
        <div
            class=move || format!(
                "flex items-center gap-2 px-2 py-1.5 cursor-pointer transition-colors {}",
                if selected.get() {
                    "bg-primary/10 border-l-2 border-primary"
                } else {
                    "hover:bg-muted"
                }
            )
            style=format!("padding-left: {}rem", depth as f32 * 1.5)
            on:click=move |_| on_select.run(node_id.clone())
        >
            // Expand/collapse arrow
            <div
                class="w-4 h-4 flex items-center justify-center text-muted-foreground"
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_toggle.run(node_id_for_toggle.clone());
                }
            >
                {if has_children {
                    if is_expanded {
                        "▼"
                    } else {
                        "▶"
                    }
                } else {
                    ""
                }}
            </div>
            
            // Icon
            {node.icon.map(|icon| view! {
                <span class="text-sm">{icon}</span>
            })}
            
            // Label
            <span class="text-sm flex-1">{node.label}</span>
        </div>
    }
}
