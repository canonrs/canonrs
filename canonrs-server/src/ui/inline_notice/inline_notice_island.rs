use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum InlineNoticeIslandVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
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
        match self {
            Self::Error => "alert",
            _           => "status",
        }
    }
    pub fn aria_live(&self) -> &'static str {
        match self {
            Self::Error => "assertive",
            _           => "polite",
        }
    }
}

#[island]
pub fn InlineNoticeIsland(
    #[prop(optional, into)] content: Option<String>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] variant: Option<InlineNoticeIslandVariant>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let variant = variant.unwrap_or_default();

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
