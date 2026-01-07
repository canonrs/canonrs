use leptos::prelude::*;
use rs_design::ui::tree::TreeNode;
use rs_design::ui::virtual_tree::VirtualTree;

/// Generate large dataset (10,000+ nodes)
fn generate_large_dataset(num_workflows: usize, steps_per_workflow: usize) -> Vec<TreeNode> {
    (0..num_workflows)
        .map(|i| {
            TreeNode::new(
                format!("workflow-{}", i),
                format!("Workflow #{}", i + 1),
                "workflow",
            )
            .with_icon("📋")
            .with_expanded(i < 2) // Only first 2 expanded by default
            .with_children(
                (0..steps_per_workflow)
                    .map(|j| {
                        TreeNode::new(
                            format!("step-{}-{}", i, j),
                            format!("Step {}", j + 1),
                            "step",
                        )
                        .with_icon(match j % 4 {
                            0 => "✅",
                            1 => "⏳",
                            2 => "🔒",
                            _ => "⏸️",
                        })
                        .with_metadata(match j % 4 {
                            0 => "Completed",
                            1 => "Active",
                            2 => "Blocked",
                            _ => "Pending",
                        })
                    })
                    .collect(),
            )
        })
        .collect()
}

#[component]
pub fn VirtualTreeTab() -> impl IntoView {
    let (nodes, set_nodes) = signal(generate_large_dataset(1000, 10)); // 10,000 nodes total
    let (selected_id, set_selected_id) = signal(None::<String>);

    let total_nodes = Signal::derive(move || {
        fn count_nodes(nodes: &[TreeNode]) -> usize {
            nodes.iter().map(|n| 1 + count_nodes(&n.children)).sum()
        }
        count_nodes(&nodes.get())
    });

    let on_select = Callback::new(move |id: String| {
        set_selected_id.set(Some(id));
    });

    let on_toggle = Callback::new(move |id: String| {
        set_nodes.update(|nodes| {
            fn toggle_node(nodes: &mut [TreeNode], target_id: &str) -> bool {
                for node in nodes.iter_mut() {
                    if node.id == target_id {
                        node.expanded = !node.expanded;
                        return true;
                    }
                    if toggle_node(&mut node.children, target_id) {
                        return true;
                    }
                }
                false
            }
            toggle_node(nodes, &id);
        });
    });

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Virtual Tree (Large Datasets)"</h2>
                <p class="text-muted-foreground">"Handles 10,000+ nodes efficiently with virtualization"</p>
            </div>

            <div class="grid grid-cols-3 gap-6">
                <div class="col-span-2">
                    <VirtualTree
                        nodes=nodes
                        selected_id=selected_id
                        on_select=on_select
                        on_toggle=on_toggle
                        row_height=36.0
                        viewport_height=600.0
                        overscan=5
                    />
                </div>

                <div class="space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Performance Stats"</h3>
                        <div class="space-y-3 text-sm">
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Total Nodes:"</span>
                                <code>{move || total_nodes.get()}</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Virtualized:"</span>
                                <code>"✅ Yes"</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Row Height:"</span>
                                <code>"36px"</code>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-muted-foreground">"Overscan:"</span>
                                <code>"5 rows"</code>
                            </div>
                        </div>
                    </div>

                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Selected Node"</h3>
                        {move || {
                            if let Some(id) = selected_id.get() {
                                view! {
                                    <div class="text-sm">
                                        <code class="break-all">{id}</code>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <p class="text-sm text-muted-foreground">"Click a node to select"</p>
                                }.into_any()
                            }
                        }}
                    </div>

                    <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                        <p class="text-sm font-semibold text-blue-900">"⚡ Virtualization"</p>
                        <ul class="text-xs text-blue-700 mt-2 space-y-1">
                            <li>"• Only visible rows rendered"</li>
                            <li>"• Smooth scrolling 60fps"</li>
                            <li>"• Low memory footprint"</li>
                            <li>"• Scales to millions of nodes"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
