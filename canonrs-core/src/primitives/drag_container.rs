//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drag Container Primitive - Wrapper for droppable zones

use leptos::prelude::*;

#[component]
pub fn DragContainerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-drag-container=""
            data-rs-state={if active { "open" } else { "closed" }}
            role="list"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
