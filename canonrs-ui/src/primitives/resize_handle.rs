//! @canon-level: strict
//! @canon-owner: primitives-team
//! Resize Handle Primitive - Visual handle for resizable columns

use leptos::prelude::*;

#[component]
pub fn ResizeHandlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = String::new())] col_id: String,
) -> impl IntoView {
    view! {
        <span
            data-resize-handle=""
            data-resize-col-id={if col_id.is_empty() { None } else { Some(col_id) }}
            role="separator"
            aria-orientation="vertical"
            class={class}
        >
            {children.map(|c| c())}
        </span>
    }
}
