use leptos::prelude::*;
use crate::components::selection::{SelectionProvider, SelectionMode, SelectableItem};
use crate::ui::button::Button;

#[component]
pub fn BulkCommandsTab() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Bulk Commands"</h2>
                <p class="text-muted-foreground">"Multi-selection + Batch operations"</p>
            </div>

            <SelectionProvider<String> mode={SelectionMode::Multiple}>
                <BulkCommandsDemo />
            </SelectionProvider<String>>
        </div>
    }
}

#[component]
fn BulkCommandsDemo() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        "Task 1".to_string(),
        "Task 2".to_string(),
        "Task 3".to_string(),
        "Task 4".to_string(),
        "Task 5".to_string(),
    ]);

    let selection_ctx = crate::components::selection::use_selection::<String>();
    let selection_ctx_delete = selection_ctx.clone();
    let selection_ctx_clear = selection_ctx.clone();
    let selected_count = Memo::new(move |_| selection_ctx.count());

    view! {
        <div class="border rounded-lg p-4">
            <div class="flex items-center justify-between mb-4 pb-3 border-b">
                <span class="text-sm font-semibold">
                    {selected_count.get()} " selected"
                </span>

                <div class="flex gap-2">
                    <Button
                        container_class="px-3 py-1 text-sm".to_string()
                        on_click=Callback::new(move |_| {
                            let selected = selection_ctx_delete.get_all();
                            leptos::logging::log!("🗑️ Delete: {:?}", selected);
                            let current = nodes.get_untracked();
                            let filtered: Vec<String> = current.into_iter()
                                .filter(|item| !selected.contains(item))
                                .collect();
                            nodes.set(filtered);
                            selection_ctx_delete.clear();
                        })
                    >
                        "🗑️ Delete"
                    </Button>

                    <Button
                        container_class="px-3 py-1 text-sm".to_string()
                        on_click=Callback::new(move |_| selection_ctx_clear.clear())
                    >
                        "Clear"
                    </Button>
                </div>
            </div>

            <div class="space-y-2">
                {move || nodes.get().into_iter().map(|item| {
                    view! {
                        <SelectableItem item={item.clone()}>
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
