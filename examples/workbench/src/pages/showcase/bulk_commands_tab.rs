use leptos::prelude::*;

#[component]
pub fn BulkCommandsTab() -> impl IntoView {
    // 3 listas separadas
    let (active_items, set_active_items) = signal(vec![
        "Task 1".to_string(),
        "Task 2".to_string(),
        "Task 3".to_string(),
        "Task 4".to_string(),
        "Task 5".to_string(),
    ]);
    
    let (archived_items, set_archived_items) = signal(Vec::<String>::new());
    let (moved_items, set_moved_items) = signal(Vec::<String>::new());

    let (selected, set_selected) = signal(std::collections::HashSet::<String>::new());

    // Keyboard shortcuts
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        
        let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new({
            let selected = selected;
            let set_active = set_active_items;
            let set_archived = set_archived_items;
            let set_moved = set_moved_items;
            let set_selected = set_selected;
            
            move |ev: web_sys::KeyboardEvent| {
                if ev.ctrl_key() {
                    let sel = selected.get();
                    if sel.is_empty() { return; }
                    
                    match ev.key().as_str() {
                        "d" => {
                            ev.prevent_default();
                            set_active.update(|items| items.retain(|item| !sel.contains(item)));
                            set_selected.set(std::collections::HashSet::new());
                            leptos::logging::log!("🗑️ Ctrl+D: Deleted {} items", sel.len());
                        }
                        "a" => {
                            ev.prevent_default();
                            let to_archive: Vec<_> = sel.iter().cloned().collect();
                            set_active.update(|items| items.retain(|item| !sel.contains(item)));
                            set_archived.update(|items| items.extend(to_archive));
                            set_selected.set(std::collections::HashSet::new());
                            leptos::logging::log!("📦 Ctrl+A: Archived {} items", sel.len());
                        }
                        "m" => {
                            ev.prevent_default();
                            let to_move: Vec<_> = sel.iter().cloned().collect();
                            set_active.update(|items| items.retain(|item| !sel.contains(item)));
                            set_moved.update(|items| items.extend(to_move));
                            set_selected.set(std::collections::HashSet::new());
                            leptos::logging::log!("📁 Ctrl+M: Moved {} items", sel.len());
                        }
                        _ => {}
                    }
                }
            }
        });
        
        let _ = web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        
        closure.forget();
    });

    view! {
        <div class="space-y-6">
            <h2 class="text-2xl font-bold">"Bulk Commands - 3 Lists"</h2>
            <p class="text-sm text-muted-foreground">"Ctrl+D=Delete | Ctrl+A=Archive | Ctrl+M=Move to Folder B"</p>
            
            <div class="grid grid-cols-3 gap-4">
                {/* Lista Ativa */}
                <div class="border rounded-lg p-4 bg-card">
                    <div class="flex justify-between items-center mb-4 pb-2 border-b">
                        <h3 class="font-semibold">"📋 Active Tasks"</h3>
                        <span class="text-xs text-muted-foreground">{move || active_items.get().len()}</span>
                    </div>
                    
                    <div class="flex gap-2 mb-4">
                        <button
                            class="px-2 py-1 text-xs border rounded hover:bg-red-50"
                            on:click=move |_| {
                                let sel = selected.get();
                                set_active_items.update(|items| items.retain(|item| !sel.contains(item)));
                                set_selected.set(std::collections::HashSet::new());
                            }
                        >
                            "🗑️"
                        </button>

                        <button
                            class="px-2 py-1 text-xs border rounded hover:bg-blue-50"
                            on:click=move |_| {
                                let sel = selected.get();
                                let to_archive: Vec<_> = sel.iter().cloned().collect();
                                set_active_items.update(|items| items.retain(|item| !sel.contains(item)));
                                set_archived_items.update(|items| items.extend(to_archive));
                                set_selected.set(std::collections::HashSet::new());
                            }
                        >
                            "📦"
                        </button>

                        <button
                            class="px-2 py-1 text-xs border rounded hover:bg-green-50"
                            on:click=move |_| {
                                let sel = selected.get();
                                let to_move: Vec<_> = sel.iter().cloned().collect();
                                set_active_items.update(|items| items.retain(|item| !sel.contains(item)));
                                set_moved_items.update(|items| items.extend(to_move));
                                set_selected.set(std::collections::HashSet::new());
                            }
                        >
                            "📁"
                        </button>
                    </div>

                    <div class="space-y-2">
                        {move || active_items.get().into_iter().map(|item| {
                            let item_for_class = item.clone();
                            let item_for_click = item.clone();
                            
                            view! {
                                <div
                                    class=move || {
                                        if selected.get().contains(&item_for_class) {
                                            "p-2 border rounded cursor-pointer bg-primary/10 border-primary text-sm"
                                        } else {
                                            "p-2 border rounded cursor-pointer hover:bg-muted text-sm"
                                        }
                                    }
                                    on:click=move |_| {
                                        set_selected.update(|s| {
                                            if !s.remove(&item_for_click) {
                                                s.insert(item_for_click.clone());
                                            }
                                        });
                                    }
                                >
                                    {item}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>

                {/* Lista Arquivada */}
                <div class="border rounded-lg p-4 bg-blue-50/50">
                    <div class="flex justify-between items-center mb-4 pb-2 border-b">
                        <h3 class="font-semibold">"📦 Archived"</h3>
                        <span class="text-xs text-muted-foreground">{move || archived_items.get().len()}</span>
                    </div>
                    <div class="space-y-2">
                        {move || archived_items.get().into_iter().map(|item| {
                            view! {
                                <div class="p-2 border rounded bg-white text-sm opacity-60">
                                    {item}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>

                {/* Lista Movida (Folder B) */}
                <div class="border rounded-lg p-4 bg-green-50/50">
                    <div class="flex justify-between items-center mb-4 pb-2 border-b">
                        <h3 class="font-semibold">"📁 Folder B"</h3>
                        <span class="text-xs text-muted-foreground">{move || moved_items.get().len()}</span>
                    </div>
                    <div class="space-y-2">
                        {move || moved_items.get().into_iter().map(|item| {
                            view! {
                                <div class="p-2 border rounded bg-white text-sm">
                                    {item}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>

            <div class="text-xs text-muted-foreground">
                {move || format!("Selected: {}", selected.get().len())}
            </div>
        </div>
    }
}
