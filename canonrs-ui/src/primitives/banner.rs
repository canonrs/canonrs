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
}

#[component]
pub fn BannerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-banner=""
            data-variant={variant.as_str()}
            data-state={if open { "open" } else { "closed" }}
            role="region"
            aria-live="polite"
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
    #[prop(default = Callback::new(|_| {}))] on_close: Callback<()>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-banner-close=""
            aria-label="Close banner"
            on:click=move |_| on_close.run(())
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
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div data-banner-content="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn BannerActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div data-banner-actions="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
