//! @canon-level: strict
//! @canon-owner: primitives-team
//! EmptyState Primitive - HTML puro

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EmptyStateVariant {
    #[default]
    Default,
    NoData,
    NoResults,
    Error,
}

impl EmptyStateVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::NoData => "no-data",
            Self::NoResults => "no-results",
            Self::Error => "error",
        }
    }
}

#[component]
pub fn EmptyStatePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-empty-state=""
            data-rs-variant=variant.as_str()
            data-rs-state="closed" data-rs-empty="true"
            role="status"
            aria-live="polite"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn EmptyStateIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-empty-state-icon=""
            aria-hidden="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn EmptyStateTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <h3 data-rs-empty-state-title="" class=class id=id>
            {children.map(|c| c())}
        </h3>
    }
}

#[component]
pub fn EmptyStateDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <p data-rs-empty-state-description="" class=class id=id>
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn EmptyStateActionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-empty-state-action="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
