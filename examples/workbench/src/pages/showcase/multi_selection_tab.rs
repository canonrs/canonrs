use leptos::prelude::*;
use rs_design::ui::selection::{SelectionProvider, SelectableItem, SelectionMode, use_selection};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Item {
    id: String,
    label: String,
    color: String,
}

#[component]
pub fn MultiSelectionTab() -> impl IntoView {
    view! {
        <SelectionProvider<String> mode=SelectionMode::Multiple>
            <MultiSelectionDemo />
        </SelectionProvider<String>>
    }
}

#[component]
fn MultiSelectionDemo() -> impl IntoView {
    let context = use_selection::<String>();
    
    let (items, set_items) = signal(vec![
        Item { id: "1".into(), label: "Item 1".into(), color: "bg-blue-100 text-blue-800".into() },
        Item { id: "2".into(), label: "Item 2".into(), color: "bg-green-100 text-green-800".into() },
        Item { id: "3".into(), label: "Item 3".into(), color: "bg-purple-100 text-purple-800".into() },
        Item { id: "4".into(), label: "Item 4".into(), color: "bg-orange-100 text-orange-800".into() },
        Item { id: "5".into(), label: "Item 5".into(), color: "bg-pink-100 text-pink-800".into() },
        Item { id: "6".into(), label: "Item 6".into(), color: "bg-red-100 text-red-800".into() },
        Item { id: "7".into(), label: "Item 7".into(), color: "bg-yellow-100 text-yellow-800".into() },
        Item { id: "8".into(), label: "Item 8".into(), color: "bg-indigo-100 text-indigo-800".into() },
    ]);
    
    let ctx_clear = context.clone();
    let clear_selection = move || {
        ctx_clear.clear();
    };
    
    let ctx_delete = context.clone();
    let delete_selected = move || {
        let selected = ctx_delete.get_all();
        
        // Remove items da lista
        set_items.update(|items| {
            items.retain(|item| !selected.contains(&item.id));
        });
        
        // Clear selection
        ctx_delete.clear();
        
        leptos::logging::log!("🗑️ Deleted {} items", selected.len());
    };
    
    let count = context.clone().count();
    let ctx_get_all = context.clone();

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Multi-Selection"</h2>
                <p class="text-muted-foreground">"Click to select, Ctrl+Click to toggle, Shift+Click to add, then delete"</p>
            </div>

            <div class="grid grid-cols-3 gap-6">
                <div class="col-span-2 space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Selectable Items"</h3>
                        
                        <div class="space-y-2">
                            {move || items.get().into_iter().map(|item| {
                                view! {
                                    <SelectableItem item=item.id.clone()>
                                        <div class=format!("p-3 rounded-lg {}", item.color)>
                                            <span class="font-medium">{item.label}</span>
                                        </div>
                                    </SelectableItem>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Actions"</h3>
                        
                        <div class="flex gap-4">
                            <button
                                class="px-4 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed"
                                disabled=move || count.get() == 0
                                on:click=move |_| delete_selected()
                            >
                                {move || format!("🗑️ Delete Selected ({})", count.get())}
                            </button>
                            
                            <button
                                class="px-4 py-2 rounded-lg border border-gray-300 hover:bg-gray-50"
                                on:click=move |_| clear_selection()
                            >
                                "Clear Selection"
                            </button>
                        </div>
                    </div>
                </div>

                <div class="space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Selection Info"</h3>
                        
                        <div class="space-y-3">
                            <div class="p-3 bg-muted rounded text-center">
                                <div class="text-3xl font-bold">{move || count.get()}</div>
                                <div class="text-sm text-muted-foreground">"Items Selected"</div>
                            </div>
                            
                            <div class="p-3 bg-muted rounded">
                                <div class="text-sm font-semibold mb-2">"Selected IDs:"</div>
                                <div class="text-xs font-mono max-h-32 overflow-y-auto">
                                    {move || {
                                        let selected = ctx_get_all.get_all();
                                        if selected.is_empty() {
                                            view! { <div class="text-muted-foreground">"None"</div> }.into_any()
                                        } else {
                                            selected.into_iter().map(|id| {
                                                view! { <div>{id}</div> }
                                            }).collect_view().into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 bg-green-50 border border-green-200 rounded">
                        <p class="text-sm font-semibold text-green-900">"✅ Features"</p>
                        <ul class="text-xs text-green-700 mt-2 space-y-1">
                            <li>"• Click: Single select"</li>
                            <li>"• Ctrl+Click: Toggle"</li>
                            <li>"• Shift+Click: Add to selection"</li>
                            <li>"• Delete selected items"</li>
                            <li>"• Real-time updates"</li>
                        </ul>
                    </div>

                    <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                        <p class="text-sm font-semibold text-blue-900">"🎯 Use Cases"</p>
                        <ul class="text-xs text-blue-700 mt-2 space-y-1">
                            <li>"• Bulk delete"</li>
                            <li>"• Bulk move"</li>
                            <li>"• Batch operations"</li>
                            <li>"• Tree multi-select"</li>
                            <li>"• List multi-select"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
