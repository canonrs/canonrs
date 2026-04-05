use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum BannerIslandVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl BannerIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info    => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error   => "error",
        }
    }
}

#[island]
pub fn BannerIsland(
    #[prop(optional, into)] content: Option<String>,
    #[prop(optional)] variant: Option<BannerIslandVariant>,
    #[prop(optional)] dismissible: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class       = class.unwrap_or_default();
    let variant     = variant.unwrap_or_default();
    let dismissible = dismissible.unwrap_or(true);
    let (is_open, set_open) = signal(true);
    let initial_state = "open";
    let state  = move || if is_open.get() { "open" } else { "closed" };
    let hidden = move || !is_open.get();
    let _ = set_open;

    #[cfg(feature = "hydrate")]
    let on_close = move |_: leptos::ev::MouseEvent| set_open.set(false);
    #[cfg(not(feature = "hydrate"))]
    let on_close = move |_: leptos::ev::MouseEvent| {};

    view! {
        <div
            data-rs-banner=""
            data-rs-component="Banner"
            data-rs-variant=variant.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            role="status"
            aria-live="polite"
            hidden=hidden
            class=class
        >
            <div data-rs-banner-content="">
                {content}
            </div>
            {dismissible.then(|| view! {
                <button
                    type="button"
                    data-rs-banner-close=""
                    aria-label="Close banner"
                    on:click=on_close
                >
                    "×"
                </button>
            })}
        </div>
    }
}
