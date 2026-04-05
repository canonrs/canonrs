use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum AlertIslandVariant {
    #[default]
    Default,
    Destructive,
    Success,
    Warning,
    Info,
}

impl AlertIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Destructive => "destructive",
            Self::Success     => "success",
            Self::Warning     => "warning",
            Self::Info        => "info",
        }
    }
    pub fn role(&self) -> &'static str {
        match self {
            Self::Destructive => "alert",
            _                 => "status",
        }
    }
}

#[island]
pub fn AlertIsland(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] variant: Option<AlertIslandVariant>,
    #[prop(optional)] dismissible: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class       = class.unwrap_or_default();
    let variant     = variant.unwrap_or_default();
    let dismissible = dismissible.unwrap_or(false);
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
            data-rs-alert=""
            data-rs-component="Alert"
            data-rs-variant=variant.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            role=variant.role()
            aria-live=if variant.role() == "alert" { "assertive" } else { "polite" }
            hidden=hidden
            class=class
        >
            <div data-rs-alert-content="">
                {title.map(|t| view! { <p data-rs-alert-title="">{t}</p> })}
                {description.map(|d| view! { <p data-rs-alert-description="">{d}</p> })}
            </div>
            {dismissible.then(|| view! {
                <button
                    type="button"
                    data-rs-alert-close=""
                    aria-label="Close"
                    on:click=on_close
                >
                    "×"
                </button>
            })}
        </div>
    }
}
