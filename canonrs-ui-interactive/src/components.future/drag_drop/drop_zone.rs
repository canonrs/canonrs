use leptos::prelude::*;
use super::types::DropTargetId;
use super::drag_context::use_drag_context;

#[component]
pub fn DropZone(
    target_id: DropTargetId,
    children: Children,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let context = use_drag_context();
    let target_id_clone = target_id.clone();
    
    let is_hovering = Memo::new(move |_| {
        context.state.get().is_hovering(&target_id_clone)
    });
    
    let target_for_over = target_id.clone();
    let target_for_drop = target_id.clone();
    
    view! {
        <div
            class={class.unwrap_or_default()}
            attr:data-drop-zone=""
            attr:data-state={if is_hovering.get() { "hovering" } else { "idle" }}
            on:dragover=move |evt| {
                evt.prevent_default();
                context.set_hover(Some(target_for_over.clone()));
            }
            on:dragleave=move |_| {
                context.set_hover(None);
            }
            on:drop=move |evt| {
                evt.prevent_default();
                context.drop_item(target_for_drop.clone());
            }
        >
            {children()}
        </div>
    }
}
