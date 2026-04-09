//! @canon-level: strict
//! @canon-owner: primitives-team
//! Alert Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Default, Debug)]
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
            Self::Default     => "default",
            Self::Destructive => "destructive",
            Self::Warning     => "warning",
            Self::Success     => "success",
        }
    }

    pub fn role(&self) -> &'static str {
        match self {
            Self::Destructive => "alert",
            _                 => "status",
        }
    }

    pub fn aria_live(&self) -> &'static str {
        match self {
            Self::Destructive => "assertive",
            _                 => "polite",
        }
    }
}

#[component]
pub fn AlertPrimitive(
    children: Children,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-alert=""
            data-rs-component="Alert"
            data-rs-behavior="notification"
            data-rs-variant=variant.as_str()
            data-rs-state="open"
            role=variant.role()
            aria-live=variant.aria_live()
            aria-atomic="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h5 data-rs-alert-title="" class=class>
            {children()}
        </h5>
    }
}

#[component]
pub fn AlertDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-alert-description="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn AlertCloseButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-alert-close=""
            aria-label="Close alert"
            class=class
        >
            {children()}
        </button>
    }
}
