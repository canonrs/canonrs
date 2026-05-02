//! @canon-level: strict
//! @canon-owner: primitives-team
//! Banner Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub enum BannerVariant {
    #[default]
    Info, Success, Warning, Error,
}
impl BannerVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info    => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error   => "error",
        }
    }
    pub fn role(&self) -> &'static str {
        match self {
            Self::Error | Self::Warning => "alert",
            _                           => "region",
        }
    }
    pub fn aria_live(&self) -> &'static str {
        match self {
            Self::Error | Self::Warning => "assertive",
            _                           => "polite",
        }
    }
    pub fn aria_label(&self) -> &'static str {
        match self {
            Self::Error   => "Error notification",
            Self::Warning => "Warning notification",
            Self::Success => "Success notification",
            Self::Info    => "System notification",
        }
    }
}

#[component]
pub fn BannerPrimitive(
    children: Children,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = VisibilityState::Open)] visibility: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("bn");
    let v = visibility_attrs(visibility);
    view! {
        <div
            data-rs-banner=""
            data-rs-uid=uid
            data-rs-interaction="dismiss"
            data-rs-variant=variant.as_str()
            data-rs-state=v.data_rs_state
            role=variant.role()
            aria-live=variant.aria_live()
            aria-label=variant.aria_label()
            aria-atomic="true"
            hidden=v.hidden
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn BannerClosePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-banner-close=""
            aria-label="Close banner"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn BannerContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-banner-content="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn BannerActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-banner-actions="" class=class>
            {children()}
        </div>
    }
}
