//! @canon-level: strict
//! @canon-owner: primitives-team
//! Popover Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn PopoverPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView
{
    view! {
        <div
            attr:data-popover=""
            attr:data-state={move || if open.get() { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn PopoverTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-popover-trigger=""
            type="button"
            attr:aria-haspopup="dialog"
            attr:aria-controls={controls_id}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn PopoverContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = "center".to_string())] align: String,
    #[prop(default = 4)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            role="dialog"
            id={content_id}
            attr:data-popover-content=""
            attr:data-align={align}
            attr:data-side-offset={side_offset}
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}
