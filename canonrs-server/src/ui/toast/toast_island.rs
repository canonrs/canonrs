use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum ToastIslandVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
    Info,
}

impl ToastIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Error   => "error",
            Self::Warning => "warning",
            Self::Info    => "info",
        }
    }
}

#[island]
pub fn ToastIsland(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] variant: Option<ToastIslandVariant>,
    #[prop(optional)] duration_ms: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class    = class.unwrap_or_default();
    let variant  = variant.unwrap_or_default();
    let duration = duration_ms.unwrap_or(5000);
    let _ = duration;
    let (is_open, set_open) = signal(true);
    let _ = set_open;
    let initial_state = "open";
    let state = move || if is_open.get() { "open" } else { "closed" };

    #[cfg(feature = "hydrate")]
    {
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsCast;
        let cb = Closure::once_into_js(move || set_open.set(false));
        leptos::web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.unchecked_ref(), duration as i32
            ).ok();
    }

    #[cfg(feature = "hydrate")]
    let on_close = move |_: leptos::ev::MouseEvent| set_open.set(false);
    #[cfg(not(feature = "hydrate"))]
    let on_close = move |_: leptos::ev::MouseEvent| {};

    view! {
        <div
            data-rs-toast=""
            data-rs-component="Toast"
            data-rs-variant=variant.as_str()
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state } else { s } }
            role="status"
            aria-live="polite"
            class=class
        >
            <div data-rs-toast-content="">
                {title.map(|t| view! { <p data-rs-toast-title="">{t}</p> })}
                {description.map(|d| view! { <p data-rs-toast-description="">{d}</p> })}
            </div>
            <button
                type="button"
                data-rs-toast-close=""
                aria-label="Close"
                on:click=on_close
            >
                "×"
            </button>
        </div>
    }
}

#[island]
pub fn ToastViewportIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <div
            data-rs-toast-viewport=""
            aria-label="Notifications"
            class=class
        >
            {children()}
        </div>
    }
}
