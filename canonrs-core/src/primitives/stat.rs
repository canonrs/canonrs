//! @canon-level: strict
//! @canon-owner: primitives-team
//! Stat Primitive - Number + label display

use leptos::prelude::*;

#[component]
pub fn StatPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-stat=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn StatValuePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-stat-value=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn StatLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-stat-label=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn StatDeltaPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-stat-delta=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn StatIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-stat-icon=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn StatHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-stat-header=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn StatBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-stat-body=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
