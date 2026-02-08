//! @canon-level: strict
//! @canon-owner: primitives-team
//! Collapsible Primitive - HTML puro

use leptos::prelude::*;
use crate::utils::id_gen::gen_collapsible_id;

#[component]
pub fn CollapsiblePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] open: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let resolved_id = if id.is_empty() { gen_collapsible_id() } else { id };

    view! {
        <div
            data-collapsible
            data-slot="collapsible"
            data-state={if open { "open" } else { "closed" }}
            class={class}
            id={resolved_id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-collapsible-trigger
            data-slot="collapsible-trigger"
            type="button"
            aria-expanded="false"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CollapsibleContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-collapsible-content
            data-slot="collapsible-content"
            role="region"
            aria-hidden="true"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
