//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drag Handle Primitive - Visual handle for draggable items

use leptos::prelude::*;

#[component]
pub fn DragHandlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-drag-handle=""
            data-rs-uid=crate::infra::uid::generate("dh")
            data-rs-state={if active { "open" } else { "closed" }}
            aria-label="Drag to reorder"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}
