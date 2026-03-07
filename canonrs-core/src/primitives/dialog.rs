//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive v2 - HTML puro + ARIA completo

use leptos::prelude::*;

#[component]
pub fn DialogPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog=id.clone()
            id=id
            data-state="closed"
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_dialog_id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            data-dialog-trigger=target_dialog_id
            type="button"
            data-button=""
            data-ui-variant="solid"
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
            data-dialog-overlay=""
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
    #[prop(into, default = String::new())] labelledby: String,
    #[prop(into, default = String::new())] describedby: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-content=""
            role="dialog"
            aria-modal="true"
            aria-labelledby={if labelledby.is_empty() { None } else { Some(labelledby.clone()) }}
            aria-describedby={if describedby.is_empty() { None } else { Some(describedby.clone()) }}
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
        <h2 data-dialog-title="" class=class id=id>
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
        <p data-dialog-description="" class=class id=id>
            {children.map(|c| c())}
        </p>
    }
}
