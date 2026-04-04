use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum CopyState {
    #[default]
    Idle,
    Copied,
    Error,
}

impl CopyState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle   => "idle",
            Self::Copied => "copied",
            Self::Error  => "error",
        }
    }
}

#[island]
pub fn CopyButtonIsland(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] target: Option<String>,
    #[prop(optional)] reset_delay: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let text        = text.unwrap_or_default();
    let target      = target.unwrap_or_default();
    let reset_delay = reset_delay.unwrap_or(2000);
    let class       = class.unwrap_or_default();
    let aria_label  = aria_label.unwrap_or_else(|| "Copy to clipboard".to_string());

    let (copy_state, set_copy_state) = signal(CopyState::Idle);
    let (is_hover, set_hover)        = signal(false);

    let initial_state = "idle".to_string();

    let state = move || {
        let mut s = vec![copy_state.get().as_str()];
        if is_hover.get() { s.push("hover"); }
        s.join(" ")
    };

    #[cfg(feature = "hydrate")]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { set_hover.set(true); };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseenter = move |_: leptos::ev::MouseEvent| { let _ = set_hover; };

    #[cfg(feature = "hydrate")]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { set_hover.set(false); };
    #[cfg(not(feature = "hydrate"))]
    let on_mouseleave = move |_: leptos::ev::MouseEvent| { let _ = set_hover; };

    #[cfg(feature = "hydrate")]
    let on_click = {
        let text = text.clone();
        let target = target.clone();
        move |_: leptos::ev::MouseEvent| {
            use leptos::wasm_bindgen::JsCast;
            use leptos::web_sys;

            let copy_text = if !text.is_empty() {
                Some(text.clone())
            } else if !target.is_empty() {
                let selector = if target.starts_with('#') {
                    target.clone()
                } else {
                    format!("#{}", target)
                };
                web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.query_selector(&selector).ok().flatten())
                    .and_then(|el| el.text_content())
            } else {
                None
            };

            match copy_text {
                Some(t) if !t.is_empty() => {
                    // clipboard write via js_sys eval (feature-safe)
                    let _ = js_sys::eval(&format!("navigator.clipboard.writeText({:?})", t));
                    set_copy_state.set(CopyState::Copied);
                    let delay = reset_delay as i32;
                    let cb = leptos::wasm_bindgen::closure::Closure::once(Box::new(move || {
                        set_copy_state.set(CopyState::Idle);
                    }) as Box<dyn FnOnce()>);
                    if let Some(window) = web_sys::window() {
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(),
                            delay,
                        );
                    }
                    cb.forget();
                }
                _ => {
                    set_copy_state.set(CopyState::Error);
                    let delay = reset_delay as i32;
                    let cb = leptos::wasm_bindgen::closure::Closure::once(Box::new(move || {
                        set_copy_state.set(CopyState::Idle);
                    }) as Box<dyn FnOnce()>);
                    if let Some(window) = web_sys::window() {
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.as_ref().unchecked_ref(),
                            delay,
                        );
                    }
                    cb.forget();
                }
            }
        }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_: leptos::ev::MouseEvent| { let _ = (&text, &target, reset_delay, set_copy_state, set_hover); };

    view! {
        <button
            class=class
            data-rs-copy-button=""
            data-rs-component="CopyButton"
            data-rs-state=move || { let s = state(); if s.is_empty() { initial_state.clone() } else { s } }
            aria-label=aria_label
            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
            on:click=on_click
        >
            <span data-rs-copy-content="">
                <svg data-rs-copy-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                    <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
                <span data-rs-copy-label="">"Copy"</span>
            </span>

            <span data-rs-copied-content="">
                <svg data-rs-copied-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M20 6 9 17l-5-5"/>
                </svg>
                <span data-rs-copied-label="" aria-live="polite">"Copied!"</span>
            </span>

            <span data-rs-error-content="">
                <svg data-rs-error-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <path d="m15 9-6 6M9 9l6 6"/>
                </svg>
                <span data-rs-error-label="" aria-live="assertive">"Failed"</span>
            </span>
        </button>
    }
}
