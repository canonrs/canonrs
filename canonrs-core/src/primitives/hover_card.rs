//! @canon-level: strict
//! @canon-owner: primitives-team
//! HoverCard Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn HoverCardPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-hover-card=""
            data-rs-state="closed"
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-hover-card-trigger=""
            tabindex="0"
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            role="tooltip"
            data-rs-hover-card-content=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
