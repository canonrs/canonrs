//! @canon-level: strict
//! @canon-owner: primitives-team
//! HoverCard Primitive - HTML puro + ARIA
//! Base: Tooltip com conteúdo rico

use leptos::prelude::*;

#[component]
pub fn HoverCardPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-hover-card=""
            data-state={move || if open.get() { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn HoverCardTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] describedby_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-hover-card-trigger=""
            tabindex="0"
            aria-describedby={describedby_id}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn HoverCardContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = 0)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            role="tooltip"
            id={content_id}
            data-hover-card-content=""
            data-side-offset={side_offset}
            aria-hidden={move || if open.get() { "false" } else { "true" }}
            tabindex="-1"
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}
