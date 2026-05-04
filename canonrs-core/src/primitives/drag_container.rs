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
    let uid_dc = crate::infra::uid::generate("dc");
    view! {
        <div
            data-rs-drag-container=""
            data-rs-uid=uid_dc
            data-rs-visibility={if active { "open" } else { "closed" }}
            role="list"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
