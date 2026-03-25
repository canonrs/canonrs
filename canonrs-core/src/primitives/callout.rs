//! @canon-level: strict
//! @canon-owner: primitives-team
//! Callout Primitive - HTML puro

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CalloutVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl CalloutVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[component]
pub fn CalloutPrimitive(
    children: Children,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <aside
            data-rs-callout=""
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </aside>
    }
}

#[component]
pub fn CalloutIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-callout-icon=""
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CalloutTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-callout-title="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CalloutDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-callout-description="" class=class>
            {children()}
        </div>
    }
}
