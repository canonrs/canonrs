//! @canon-level: strict
//! @canon-owner: primitives-team
//! InlineNotice Primitive - HTML puro

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum InlineNoticeVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl InlineNoticeVariant {
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
pub fn InlineNoticePrimitive(
    children: Children,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-inline-notice=""
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InlineNoticeIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-inline-notice-icon=""
            aria-hidden="true"
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn InlineNoticeContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-inline-notice-content=""
            class=class
        >
            {children()}
        </span>
    }
}
