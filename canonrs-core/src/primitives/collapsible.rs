//! @canon-level: strict
//! @canon-owner: primitives-team
//! Collapsible Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CollapsiblePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-collapsible=""
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-collapsible-trigger=""
            aria-expanded={if open { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CollapsibleContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-collapsible-content=""
            data-rs-state={if open { "open" } else { "closed" }}
            role="region"
            aria-hidden={if open { "false" } else { "true" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
