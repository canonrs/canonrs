//! @canon-level: strict
//! @canon-owner: primitives-team
//! Stat Primitive - Number + label display

use leptos::prelude::*;

#[component]
pub fn StatPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-stat=""
            data-rs-component="Stat"
            data-rs-behavior="display"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn StatValuePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-value="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-label="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatDeltaPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-delta="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-icon="" aria-hidden="true" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-stat-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-stat-body="" class=class>
            {children()}
        </div>
    }
}
