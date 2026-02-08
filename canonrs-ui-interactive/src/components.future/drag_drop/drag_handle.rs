use leptos::prelude::*;
use super::types::DragItemId;
use super::drag_context::use_drag_context;

#[component]
pub fn DragHandle(
    item_id: DragItemId,
    #[prop(optional)] data: Option<String>,
    children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let context = use_drag_context();
    let item_id_clone = item_id.clone();
    let data_clone = data.clone();
    
    view! {
        <div
            class={class.unwrap_or_default()}
            draggable="true"
            attr:data-drag-handle=""
            on:dragstart=move |_| {
                context.start_drag(item_id_clone.clone(), data_clone.clone());
            }
        >
            {children()}
        </div>
    }
}
