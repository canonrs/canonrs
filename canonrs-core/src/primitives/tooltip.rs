//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tooltip Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TooltipPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip=""
            data-state="closed"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TooltipTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_tooltip_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-tooltip-trigger={target_tooltip_id}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TooltipContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip-content=""
            role="tooltip"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TooltipProviderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tooltip-provider="" class=class>
            {children.map(|c| c())}
        </div>
    }
}
