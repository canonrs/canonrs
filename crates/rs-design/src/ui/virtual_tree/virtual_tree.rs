use leptos::prelude::*;
use crate::ui::tree::TreeNode;
use super::types::VirtualTreeNode;
use super::flattener::flatten_tree;
use super::viewport::VirtualTreeViewport;

/// VirtualTree - High-performance tree for large datasets
///
/// **Type:** Stateful Component (Type 2)
/// **Pattern:** Virtualized rendering (only visible nodes)
/// **Tokens:** 100% Canonical
/// **Performance:** Handles 10,000+ nodes efficiently
#[component]
pub fn VirtualTree(
    /// Hierarchical tree nodes
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

    /// Row height in pixels
    #[prop(default = 36.0)]
    row_height: f64,

    /// Viewport height in pixels
    #[prop(default = 600.0)]
    viewport_height: f64,

    /// Number of rows to render outside viewport (performance tuning)
    #[prop(default = 5)]
    overscan: usize,
) -> impl IntoView {
    // Flatten hierarchical tree into flat list for virtualization
    let flat_nodes = Signal::derive(move || {
        flatten_tree(&nodes.get())
    });

    let total_visible = Signal::derive(move || {
        flat_nodes.get().len()
    });

    view! {
        <div class="w-full bg-background border border-border rounded-lg overflow-hidden">
            <div class="p-3 bg-muted border-b flex items-center justify-between">
                <h3 class="font-semibold text-sm">"Virtual Tree"</h3>
                <span class="text-xs text-muted-foreground">
                    {move || format!("{} visible", total_visible.get())}
                </span>
            </div>

            <VirtualTreeViewport
                nodes=flat_nodes
                selected_id=selected_id
                on_select=on_select
                on_toggle=on_toggle
                row_height=row_height
                viewport_height=viewport_height
                overscan=overscan
            />
        </div>
    }
}
