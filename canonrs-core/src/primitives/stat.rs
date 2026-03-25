//! @canon-level: strict
//! @canon-owner: primitives-team
//! Stat Primitive - Number + label display

use leptos::prelude::*;

#[component]
pub fn StatPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-stat=""
            class={class}
            id={id}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn StatValuePrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-stat-value=""
            class={class}
            id={id}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn StatLabelPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-stat-label=""
            class={class}
            id={id}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn StatDeltaPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-stat-delta=""
            class={class}
            id={id}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn StatIconPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-stat-icon=""
            class={class}
            id={id}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn StatHeaderPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-stat-header=""
            class={class}
            id={id}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn StatBodyPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-stat-body=""
            class={class}
            id={id}
        >
            {children()}
        </div>
    }
}
