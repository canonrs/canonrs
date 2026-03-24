//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive - HTML puro + ARIA completo

use leptos::prelude::*;

#[component]
pub fn DialogPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog=""
            data-rs-state="closed"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dialog-trigger=""
            aria-haspopup="dialog"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DialogOverlayPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-overlay=""
            aria-hidden="true"
            class=class
            id=id
        />
    }
}

#[component]
pub fn DialogContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dialog-content=""
            role="dialog"
            aria-modal="true"
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            tabindex="-1"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <h2 data-rs-dialog-title="" class=class id=id>
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn DialogDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <p data-rs-dialog-description="" class=class id=id>
            {children.map(|c| c())}
        </p>
    }
}
