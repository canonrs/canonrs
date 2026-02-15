//! @canon-level: strict
//! @canon-owner: primitives-team
//! CodeBlock Primitive - HTML puro, SSR-safe

use leptos::prelude::*;

#[component]
pub fn CodeBlockPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] language: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-code-block=""
            data-language=language
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CodeBlockHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-code-header="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CodeBlockLanguagePrimitive(
    #[prop(into)] language: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-code-language="" class=class>
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
        <span data-code-filename="" class=class>
            {filename}
        </span>
    }
}

#[component]
pub fn CodeBlockCopyButtonPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-code-copy-btn=""
            aria-label="Copy code"
            class=class
        >
            <span data-code-copy-label="">"Copy"</span>
        </button>
    }
}

#[component]
pub fn CodeBlockPrePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <pre data-code-pre="" class=class>
            {children.map(|c| c())}
        </pre>
    }
}

#[component]
pub fn CodeBlockLinePrimitive(
    #[prop(into)] html: String,
    #[prop(default = 0usize)] line_number: usize,
    #[prop(into, default = String::new())] diff: String,
) -> impl IntoView {
    view! {
        <span
            data-code-line=""
            data-line-number=line_number.to_string()
            attr:data-diff={if diff.is_empty() { None } else { Some(diff) }}
            inner_html=html
        ></span>
    }
}
