//! @canon-level: strict
//! @canon-owner: primitives-team
//! CodeBlock Primitive - HTML puro, SSR-safe

use leptos::prelude::*;
use crate::meta::ToggleState;
use crate::infra::state_engine::toggle_attrs;

#[component]
pub fn CodeBlockPrimitive(
    children: Children,
    #[prop(into, default = String::new())] language: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-code-block=""
            data-rs-uid=crate::infra::uid::generate("cob")
            data-rs-interaction="content"
            data-rs-language=language
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CodeBlockHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-code-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CodeBlockLanguagePrimitive(
    #[prop(into)] language: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-code-language="" class=class>
            {language}
        </span>
    }
}

#[component]
pub fn CodeBlockFilenamePrimitive(
    #[prop(into)] filename: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-code-filename="" class=class>
            {filename}
        </span>
    }
}

#[component]
pub fn CodeBlockCopyButtonPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] copied: ToggleState,
) -> impl IntoView {
    let ta = toggle_attrs(copied);
    view! {
        <button
            type="button"
            data-rs-code-copy-btn=""
            data-rs-state=ta.data_rs_state
            aria-pressed=ta.aria_pressed
            aria-label="Copy code"
            class=class
        >
            <span data-rs-code-copy-label="">"Copy"</span>
        </button>
    }
}

#[component]
pub fn CodeBlockPrePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] inner_html: String,
) -> impl IntoView {
    if inner_html.is_empty() {
        view! {
            <pre data-rs-code-pre="" class=class>
                {children.map(|c| c())}
            </pre>
        }.into_any()
    } else {
        view! {
            <pre data-rs-code-pre="" class=class inner_html=inner_html></pre>
        }.into_any()
    }
}

#[component]
pub fn CodeBlockLinePrimitive(
    #[prop(into)] _html: String,
    #[prop(default = 0usize)] line_number: usize,
    #[prop(into, default = String::new())] diff: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        view! {
            <span
                data-rs-code-line=""
                data-rs-line-number=line_number.to_string()
                data-rs-diff={if diff.is_empty() { None } else { Some(diff) }}
                inner_html=_html
            ></span>
        }.into_any()
    }
    #[cfg(not(feature = "ssr"))]
    {
        view! {
            <span
                data-rs-code-line=""
                data-rs-line-number=line_number.to_string()
                data-rs-diff={if diff.is_empty() { None } else { Some(diff.clone()) }}
            ></span>
        }.into_any()
    }
}
