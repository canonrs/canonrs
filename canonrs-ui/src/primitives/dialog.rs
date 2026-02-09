//! @canon-level: strict
//! @canon-owner: primitives-team
//! Dialog Primitive - HTML puro + ARIA (padrão unificado)

use leptos::prelude::*;

#[component]
pub fn DialogPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-dialog="" data-state="closed" class=class id=id>
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
        <div data-dialog-trigger={target_dialog_id} class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DialogOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-dialog-overlay="" aria-hidden="true" class=class id=id />
    }
}

#[component]
pub fn DialogContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-dialog-content="" role="dialog" class=class id=id>
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
        <h2 data-dialog-title="" class=class id=id>
            {children.map(|c| c())}
        </h2>
    }
}

#[component]
pub fn DialogDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <p data-dialog-description="" class=class id=id>
            {children.map(|c| c())}
        </p>
    }
}
