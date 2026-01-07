use leptos::prelude::*;
use rs_design::ui::tree::TreeNode;
use rs_design::ui::selection::{SelectionProvider, SelectionMode, SelectableItem};

#[component]
pub fn BulkCommandsTab() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Bulk Commands"</h2>
                <p class="text-muted-foreground">"Multi-selection + Batch operations"</p>
            </div>

            <SelectionProvider mode=SelectionMode::Multiple>
                <BulkCommandsDemo />
            </SelectionProvider>
        </div>
    }
}

#[component]
fn BulkCommandsDemo() -> impl IntoView {
    let (nodes, set_nodes) = signal(vec![
        "Task 1".to_string(),
        "Task 2".to_string(),
        "Task 3".to_string(),
        "Task 4".to_string(),
        "Task 5".to_string(),
    ]);

    let selection_ctx = rs_design::ui::selection::use_selection::<String>();
    let selected_count = selection_ctx.count();

    view! {
        <div class="border rounded-lg p-4">
            {/* Action Bar */}
            <div class="flex items-center justify-between mb-4 pb-3 border-b">
                <span class="text-sm font-semibold">
                    {move || format!("{} selected", selected_count.get())}
                </span>

                <div class="flex gap-2">
                    <button
                        class="px-3 py-1 text-sm border rounded hover:bg-destructive/10"
                        on:click=move |_| {
                            let selected = selection_ctx.get_all();
                            eprintln!("🗑️ Delete: {:?}", selected);
                            set_nodes.update(|n| n.retain(|item| !selected.contains(item)));
                            selection_ctx.clear();
                        }
                    >
                        "🗑️ Delete"
                    </button>

                    <button
                        class="px-3 py-1 text-sm border rounded hover:bg-muted"
                        on:click=move |_| selection_ctx.clear()
                    >
                        "Clear"
                    </button>
                </div>
            </div>

            {/* Items */}
            <div class="space-y-2">
                {move || nodes.get().into_iter().map(|item| {
                    view! {
                        <SelectableItem item=item.clone()>
                            <div class="p-3 border rounded cursor-pointer hover:bg-muted">
                                {item}
                            </div>
                        </SelectableItem>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
