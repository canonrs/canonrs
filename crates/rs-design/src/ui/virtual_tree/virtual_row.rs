use leptos::prelude::*;
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
    let node_id_for_toggle = node.id.clone();
    let indent = node.depth as f64 * 1.5;

    view! {
        <div
            class=move || format!(
                "flex items-center gap-2 px-2 cursor-pointer transition-colors {}",
                if selected {
                    "bg-primary/10 border-l-2 border-primary"
                } else {
                    "hover:bg-muted"
                }
            )
            style=format!("height: {}px; padding-left: {}rem", row_height, indent)
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
                {if node.has_children {
                    if node.expanded {
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

            // Metadata badge
            {node.metadata.map(|meta| view! {
                <span class="text-xs px-2 py-0.5 rounded bg-muted text-muted-foreground">
                    {meta}
                </span>
            })}
        </div>
    }
}
