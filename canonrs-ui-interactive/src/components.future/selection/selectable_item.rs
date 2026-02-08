use leptos::prelude::*;
use super::selection_context::use_selection;

#[component]
pub fn SelectableItem<T>(
    item: T,
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    let context = use_selection::<T>();
    let item_clone = item.clone();
    let is_selected = context.is_selected(&item_clone);

    view! {
        <div
            attr:data-selectable-item=""
            attr:data-selected={is_selected.to_string()}
            class={class}
            id={id}
            on:click=move |_| {
                context.toggle(item.clone());
            }
        >
            {children()}
        </div>
    }
}
