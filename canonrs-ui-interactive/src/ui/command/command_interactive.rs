use leptos::web_sys;
use leptos::prelude::*;
use canonrs_ui::primitives::{
    CommandPrimitive, CommandInputPrimitive, CommandListPrimitive,
    CommandItemPrimitive, CommandEmptyPrimitive,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CommandItemData {
    pub id: String,
    pub label: String,
    pub value: String,
    pub keywords: Vec<String>,
    pub group: Option<String>,
}

#[component]
pub fn CommandInteractive(
    items: Vec<CommandItemData>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] multi_select: bool,
) -> impl IntoView {
    let search_query = RwSignal::new(String::new());
    let focused_index = RwSignal::new(0);
    let selected_items = RwSignal::new(Vec::<String>::new());
    
    let items_clone = items.clone();
    
    // Usar Memo ao invés de closure para evitar move
    let filtered_items = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            items_clone.clone()
        } else {
            items_clone
                .iter()
                .filter(|item| {
                    item.label.to_lowercase().contains(&query)
                        || item.value.to_lowercase().contains(&query)
                        || item.keywords.iter().any(|k| k.to_lowercase().contains(&query))
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    });
    
    // Highlight matches no texto
    let highlight_text = move |text: &str, query: &str| {
        if query.is_empty() {
            return text.to_string();
        }
        
        let lower_text = text.to_lowercase();
        let lower_query = query.to_lowercase();
        
        if let Some(pos) = lower_text.find(&lower_query) {
            let before = &text[..pos];
            let matched = &text[pos..pos + query.len()];
            let after = &text[pos + query.len()..];
            format!("{}<mark>{}</mark>{}", before, matched, after)
        } else {
            text.to_string()
        }
    };
    
    // Keyboard navigation
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let current = focused_index.get();
        let items_len = filtered_items.get().len();
        
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if current < items_len.saturating_sub(1) {
                    focused_index.set(current + 1);
                }
            }
            "ArrowUp" => {
                ev.prevent_default();
                if current > 0 {
                    focused_index.set(current - 1);
                }
            }
            "Enter" => {
                ev.prevent_default();
                if let Some(item) = filtered_items.get().get(current) {
                    if multi_select {
                        selected_items.update(|s| {
                            if s.contains(&item.id) {
                                s.retain(|id| id != &item.id);
                            } else {
                                s.push(item.id.clone());
                            }
                        });
                    }
                    if let Some(cb) = on_select {
                        cb.run(item.value.clone());
                    }
                }
            }
            "Escape" => {
                ev.prevent_default();
                search_query.set(String::new());
                focused_index.set(0);
            }
            _ => {}
        }
    };
    
    view! {
        <CommandPrimitive>
            <input
                data-command-input=""
                type="text"
                placeholder=placeholder.unwrap_or_else(|| "Search...".to_string())
                on:input=move |ev| {
                    search_query.set(event_target_value(&ev));
                    focused_index.set(0);
                }
                on:keydown=handle_keydown
                prop:value=move || search_query.get()
                style="width: 100%; padding: 0.75rem; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); font-size: 0.875rem; background: var(--theme-surface-bg); outline: none;"
            />
            
            <CommandListPrimitive>
                {move || {
                    let items_list = filtered_items.get();
                    let query = search_query.get();
                    let query_clone = query.clone();
                    
                    if items_list.is_empty() {
                        view! {
                            <CommandEmptyPrimitive>
                                <div style="padding: 2rem; text-align: center; color: var(--theme-surface-fg-muted);">
                                    "No results found."
                                </div>
                            </CommandEmptyPrimitive>
                        }.into_any()
                    } else {
                        items_list.into_iter().enumerate().map(|(index, item)| {
                            let query_str = query_clone.clone();
                            let is_focused = index == focused_index.get();
                            let is_selected = selected_items.get().contains(&item.id);
                            let item_id = item.id.clone();
                            let item_value = item.value.clone();
                            
                            view! {
                                <CommandItemPrimitive
                                    class={if is_focused { "focused".to_string() } else { String::new() }}
                                    class:selected={is_selected}
                                >
                                    <div
                                        on:click=move |_| {
                                            if multi_select {
                                                let id = item_id.clone();
                                                selected_items.update(|s| {
                                                    if s.contains(&id) {
                                                        s.retain(|i| i != &id);
                                                    } else {
                                                        s.push(id);
                                                    }
                                                });
                                            }
                                            if let Some(cb) = on_select {
                                                cb.run(item_value.clone());
                                            }
                                        }
                                        style="padding: 0.75rem; cursor: pointer; display: flex; align-items: center; gap: 0.5rem;"
                                    >
                                        {if multi_select {
                                            view! {
                                                <input
                                                    type="checkbox"
                                                    checked=is_selected
                                                    on:click=|e| e.stop_propagation()
                                                />
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                        <span inner_html={highlight_text(&item.label, &query_str)}></span>
                                    </div>
                                </CommandItemPrimitive>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
            </CommandListPrimitive>
        </CommandPrimitive>
    }
}
