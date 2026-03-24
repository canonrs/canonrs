//! @canon-level: strict
//! @canon-owner: primitives-team
//! Resize Handle Primitive - Visual handle for resizable columns

use leptos::prelude::*;

#[component]
pub fn ResizeHandlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] col_id: Option<String>,
) -> impl IntoView {
    view! {
        <span
            data-rs-resize-handle=""
            data-rs-state="inactive"
            data-rs-resize-col-id=col_id
            role="separator"
            aria-orientation="vertical"
            tabindex="0"
            class=class
        >
            {children.map(|c| c())}
        </span>
    }
}
