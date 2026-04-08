//! InlineNotice Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum InlineNoticeIslandVariant {
    #[default] Default,
    Info, Success, Warning, Error,
}

impl InlineNoticeIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info    => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error   => "error",
        }
    }
    pub fn role(&self) -> &'static str {
        match self { Self::Error => "alert", _ => "status" }
    }
    pub fn aria_live(&self) -> &'static str {
        match self { Self::Error => "assertive", _ => "polite" }
    }
}

#[component]
pub fn InlineNoticeIsland(
    #[prop(optional, into)] content: Option<String>,
    #[prop(optional, into)] icon:    Option<String>,
    #[prop(default = InlineNoticeIslandVariant::Default)] variant: InlineNoticeIslandVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-inline-notice=""
            data-rs-component="InlineNotice"
            data-rs-variant=variant.as_str()
            role=variant.role()
            aria-live=variant.aria_live()
            class=class
        >
            {icon.map(|i| view! { <span data-rs-inline-notice-icon="">{i}</span> })}
            <span data-rs-inline-notice-content="">{content}</span>
        </div>
    }
}
