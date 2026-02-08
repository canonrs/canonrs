//! @canon-level: strict
//! @canon-owner: primitives-team
//! EmptyState Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn EmptyStatePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-empty-state=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn EmptyStateIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-empty-state-icon=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn EmptyStateTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <h3
            data-empty-state-title=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </h3>
    }
}

#[component]
pub fn EmptyStateDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <p
            data-empty-state-description=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn EmptyStateActionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-empty-state-action=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
