//! @canon-level: strict
//! @canon-owner: primitives-team
//! Callout Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CalloutPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <aside
            data-callout=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </aside>
    }
}

#[component]
pub fn CalloutIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-icon=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalloutTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-title=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalloutDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-description=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
