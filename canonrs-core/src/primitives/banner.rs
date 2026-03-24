//! @canon-level: strict
//! @canon-owner: primitives-team
//! Banner Primitive - HTML puro

use leptos::prelude::*;

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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let role = variant.role();
    let aria_live = variant.aria_live();
    view! {
        <div
            data-rs-banner=""
            data-rs-variant=variant.as_str()
            data-rs-state={if open { "open" } else { "closed" }}
            role=role
            aria-live=aria_live
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn BannerClosePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-banner-close=""
            aria-label="Close banner"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn BannerContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-banner-content="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn BannerActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-banner-actions="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
