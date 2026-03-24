//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drag Handle Primitive - Visual handle for draggable items

use leptos::prelude::*;

#[component]
pub fn DragHandlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-drag-handle=""
            data-rs-state="closed"
            aria-label="Drag to reorder"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}
