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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <aside
            data-callout=""
            data-variant={variant.as_str()}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </aside>
    }
}

#[component]
pub fn CalloutIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-icon=""
            aria-hidden="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalloutTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-title=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CalloutDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-callout-description=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
