//! @canon-level: strict
//! @canon-owner: primitives-team
//! CodeBlock Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CodeBlockPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] language: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-code-block=""
            attr:data-language={language.unwrap_or_default()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CodeBlockHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-code-header="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CodeBlockLanguagePrimitive(
    language: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span data-code-language="" class={class} id={id}>
            {language}
        </span>
    }
}

#[component]
pub fn CodeBlockContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <pre data-code-pre="" class={class.clone()} id={id.clone()}>
            <code data-code-content="" class={class.clone()} id={id.clone()}>
                {children.map(|c| c())}
            </code>
        </pre>
    }
}
