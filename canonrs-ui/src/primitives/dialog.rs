//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn DialogPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog=""
            data-state="closed"
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
    #[prop(into)] target_dialog_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-dialog-trigger={target_dialog_id}
            type="button"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DialogBackdropPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-backdrop=""
            class=class
            id=id
        />
    }
}

#[component]
pub fn DialogPopupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-popup=""
            role="dialog"
            aria-modal="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-header=""
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-dialog-title="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-dialog-body=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogClosePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_dialog_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-dialog-close={target_dialog_id}
            type="button"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DialogDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-dialog-description="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}
