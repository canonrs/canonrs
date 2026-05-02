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
            Self::Default   => "default",
            Self::NoData    => "no-data",
            Self::NoResults => "no-results",
            Self::Error     => "error",
        }
    }
}

#[component]
pub fn EmptyStatePrimitive(
    children: Children,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_es = crate::infra::uid::generate("es");
    view! {
        <div
            data-rs-empty=""
            data-rs-uid=uid_es
            data-rs-variant=variant.as_str()
            role="status"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyStateIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-empty-icon="" aria-hidden="true" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyStateTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h3 data-rs-empty-title="" class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn EmptyStateDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-empty-description="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn EmptyStateActionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-empty-action="" class=class>
            {children()}
        </div>
    }
}
