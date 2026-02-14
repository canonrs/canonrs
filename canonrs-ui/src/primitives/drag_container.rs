//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drag Container Primitive - Wrapper for droppable zones

use leptos::prelude::*;

#[component]
pub fn DragContainerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-drag-container=""
            role="list"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
