use leptos::prelude::*;
use super::list_item_ui::{List, ListItem, ListItemTitle, ListItemDescription, ListSelectionMode};

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListIslandItem {
    pub label:       String,
    pub description: Option<String>,
    pub disabled:    bool,
}

#[island]
pub fn ListItemIsland(
    items: Vec<ListIslandItem>,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let multiple       = multiple.unwrap_or(false);
    let class          = class.unwrap_or_default();
    let selection_mode = if multiple { ListSelectionMode::Multiple } else { ListSelectionMode::Single };

    let selected     = RwSignal::new(Vec::<usize>::new());
    let hovered      = RwSignal::new(Option::<usize>::None);
    let stored_items = StoredValue::new(items);

    view! {
        <List _selection_mode=selection_mode class=class>
            {move || {
                stored_items.get_value().into_iter().enumerate().map(|(i, item)| {
                    let disabled = item.disabled;
                    let label    = item.label.clone();
                    let desc     = item.description.clone();

                    view! {
                        <ListItem
                            selectable=true
                            selected=false
                            disabled=disabled
                            on:click=move |_| {
                                if disabled { return; }
                                selected.update(|sel| {
                                    if multiple {
                                        if sel.contains(&i) { sel.retain(|&x| x != i); }
                                        else { sel.push(i); }
                                    } else {
                                        if sel.contains(&i) { sel.clear(); }
                                        else { *sel = vec![i]; }
                                    }
                                });
                            }
                            on:mouseenter=move |_| { hovered.set(Some(i)); }
                            on:mouseleave=move |_| { hovered.set(None); }
                        >
                            <div
                                data-rs-list-item-content=""
                                data-rs-selectable=""
                                data-rs-state=move || {
                                    let is_selected = selected.get().contains(&i);
                                    let is_hovered  = hovered.get() == Some(i);
                                    match (is_selected, is_hovered, disabled) {
                                        (_, _, true)     => "disabled",
                                        (true, true, _)  => "selected hover",
                                        (true, false, _) => "selected",
                                        (false, true, _) => "hover",
                                        _                => "",
                                    }
                                }
                            >
                                <ListItemTitle>{label}</ListItemTitle>
                                {desc.map(|d| view! { <ListItemDescription>{d}</ListItemDescription> })}
                            </div>
                        </ListItem>
                    }
                }).collect::<Vec<_>>()
            }}
        </List>
    }
}
