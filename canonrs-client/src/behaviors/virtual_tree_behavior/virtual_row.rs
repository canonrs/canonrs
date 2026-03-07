use leptos::prelude::*;
use crate::primitives::tree::TreeItem as TreeItemPrimitive;
use super::types::VirtualTreeNode;

#[component]
pub fn VirtualTreeRow(
    node: VirtualTreeNode,
    row_height: f64,
    selected: bool,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_toggle = node.id.clone();
    
    view! {
        <TreeItemPrimitive
            depth={node.depth}
            selected={selected}
            expanded={node.expanded}
            has_children={node.has_children}
            attr:data-row-height={row_height.to_string()}
            on:click=move |_| on_select.run(node_id.clone())
        >
            // Expand/collapse toggle
            <div
                attr:data-tree-toggle=""
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_toggle.run(node_id_toggle.clone());
                }
            >
                {if node.has_children {
                    if node.expanded { "▼" } else { "▶" }
                } else {
                    ""
                }}
            </div>
            
            // Icon
            {node.icon.map(|icon| view! {
                <span data-tree-icon="">{icon}</span>
            })}
            
            // Label
            <span data-tree-label="">{node.label}</span>
            
            // Metadata
            {node.metadata.map(|meta| view! {
                <span data-tree-metadata="">{meta}</span>
            })}
        </TreeItemPrimitive>
    }
}
