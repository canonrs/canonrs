//! @canon-level: strict
//! @canon-owner: primitives-team
//! Alert Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Warning,
    Success,
}

impl AlertVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
            Self::Warning => "warning",
            Self::Success => "success",
        }
    }
}

#[component]
pub fn AlertPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-alert=""
            data-rs-variant=variant.as_str()
            role="alert"
            aria-live="polite"
            aria-atomic="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn AlertTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <h5 data-rs-alert-title="" class=class id=id>
            {children.map(|c| c())}
        </h5>
    }
}

#[component]
pub fn AlertDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-alert-description="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
