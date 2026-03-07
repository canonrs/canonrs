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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-inline-notice=""
            data-variant={variant.as_str()}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn InlineNoticeIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <span
            data-inline-notice-icon=""
            aria-hidden="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn InlineNoticeContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <span
            data-inline-notice-content=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}
