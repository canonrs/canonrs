//! @canon-level: strict
//! @canon-owner: primitives-team
//! Banner Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BannerVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl BannerVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    pub fn role(&self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }

    pub fn aria_live(&self) -> &'static str {
        match self {
            Self::Error => "assertive",
            _ => "polite",
        }
    }
}

#[component]
pub fn BannerPrimitive(
    children: Children,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let role = variant.role();
    let aria_live = variant.aria_live();
    view! {
        <div
            data-rs-banner=""
            data-rs-variant=variant.as_str()
            data-rs-state=s.data_rs_state
            role=role
            aria-live=aria_live
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
