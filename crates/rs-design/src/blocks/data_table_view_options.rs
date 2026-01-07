//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Fixed width for table options
//! @canon-owner: data-team
//! @canon-target-date: 2025-03-15

use leptos::prelude::*;
use crate::ui::button::{ButtonVariant, ButtonType};
use crate::ui::*;
use std::collections::HashMap;

#[component]
pub fn DataTableViewOptions(
    columns: Vec<String>,
    visible_columns: RwSignal<HashMap<String, bool>>,
) -> impl IntoView {
    let columns = StoredValue::new(columns);
    
    // Detecção client-side: usa window para saber se está no browser
    let is_browser = RwSignal::new(false);
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if web_sys::window().is_some() {
            is_browser.set(true);
        }
    });

    view! {
        <Show
            when=move || is_browser.get()
            fallback=|| view! {
                <div class="ml-auto hidden h-8 lg:flex w-20"></div>
            }
        >
            <DropdownMenu>
                <DropdownMenuTrigger>
                    <Button
                        variant=ButtonVariant::Outline
                        class="ml-auto hidden h-8 lg:flex gap-2"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
                            <circle cx="12" cy="12" r="3"/>
                        </svg>
                        "View"
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent class="w-[150px]">
                    <DropdownMenuLabel>
                        "Toggle columns"
                    </DropdownMenuLabel>
                    <DropdownMenuSeparator />
                    {move || {
                        columns.get_value()
                            .iter()
                            .cloned()
                            .map(|column| {
                                let col_id = StoredValue::new(column.clone());
                                let col_name = StoredValue::new(column.clone());
                                
                                let is_visible = RwSignal::new(
                                    visible_columns.get_untracked()
                                        .get(&column)
                                        .copied()
                                        .unwrap_or(true)
                                );

                                Effect::new(move |_| {
                                    let checked = is_visible.get();
                                    let id = col_id.get_value();
                                    visible_columns.update(|map| {
                                        map.insert(id, checked);
                                    });
                                });

                                view! {
                                    <DropdownMenuCheckboxItem
                                        checked=is_visible
                                        class="capitalize"
                                    >
                                        {move || col_name.get_value()}
                                    </DropdownMenuCheckboxItem>
                                }
                            })
                            .collect_view()
                    }}
                </DropdownMenuContent>
            </DropdownMenu>
        </Show>
    }
}
