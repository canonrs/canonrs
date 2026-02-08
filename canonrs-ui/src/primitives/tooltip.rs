//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tooltip Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TooltipProviderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0)] delay_duration: u32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-tooltip-provider=""
            data-delay={delay_duration}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TooltipPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView
{
    view! {
        <div
            data-tooltip=""
            data-state={move || if open.get() { "open" } else { "closed" }}
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
    #[prop(default = String::new())] describedby_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-tooltip-trigger=""
            type="button"
            aria-describedby={describedby_id}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn TooltipContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = 0)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            role="tooltip"
            id={content_id}
            data-tooltip-content=""
            data-side-offset={side_offset}
            class=class
        >
            {children.map(|c| c())}
            <div data-tooltip-arrow="" />
        </div>
    }
}
