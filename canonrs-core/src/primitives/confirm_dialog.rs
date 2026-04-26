//! @canon-level: strict
//! @canon-owner: primitives-team
//! ConfirmDialog Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ConfirmDialogVariant {
    #[default]
    Default,
    Destructive,
    Warning,
}
impl ConfirmDialogVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "primary",
            Self::Destructive => "destructive",
            Self::Warning     => "warning",
        }
    }
}

#[component]
pub fn ConfirmDialogPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = leptos::prelude::Signal::derive(|| false))] open: leptos::prelude::Signal<bool>,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] uid: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let computed_state = move || {
        if open.get() { "open" } else { s.data_rs_state }
    };
    let uid_str = if uid.is_empty() { crate::infra::uid::generate("cd") } else { uid };
    provide_context(open);
    provide_context(uid_str.clone());
    view! {
        <div
            data-rs-confirm-dialog=""
            data-rs-interaction="overlay"
            data-rs-uid=uid_str
            data-rs-variant=variant.as_str()
            data-rs-state=computed_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let open = use_context::<Signal<bool>>().unwrap_or(Signal::derive(|| false));
    let computed_state = move || {
        if open.get() { "open" } else { s.data_rs_state }
    };
    view! {
        <div
            data-rs-confirm-dialog-overlay=""
            data-rs-state=computed_state
            class=class
        />
    }
}

#[component]
pub fn ConfirmDialogTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h2 data-rs-confirm-dialog-title="" class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn ConfirmDialogDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-confirm-dialog-description="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-confirm-dialog-actions=""
            role="group"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogCancelPrimitive(
    children: Children,
    #[prop(into, default = "Cancel".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-cancel=""
            data-rs-button=""
            data-rs-variant="ghost"
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogConfirmPrimitive(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = "Confirm".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-confirm=""
            data-rs-button=""
            data-rs-variant=variant.as_str()
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogTriggerPrimitive(
    children: Children,
    #[prop(default = ConfirmDialogVariant::Default)] variant: ConfirmDialogVariant,
    #[prop(into, default = String::new())] target: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    // lê uid do context se target não foi passado explicitamente
    let ctx_uid = use_context::<String>().unwrap_or_default();
    let resolved_target = if target.is_empty() { ctx_uid } else { target };
    view! {
        <button
            type="button"
            data-rs-confirm-dialog-trigger=""
            data-rs-button=""
            data-rs-variant=variant.as_str()
            data-rs-target=resolved_target
            aria-haspopup="alertdialog"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ConfirmDialogPortalPrimitive(
    children: ChildrenFn,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let class = StoredValue::new_local(class);
    view! {
        <leptos::portal::Portal>
            <div data-rs-confirm-dialog-portal="" class=class.get_value()>
                {children()}
            </div>
        </leptos::portal::Portal>
    }
}

#[component]
pub fn ConfirmDialogContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let open = use_context::<Signal<bool>>().unwrap_or(Signal::derive(|| false));
    let computed_state = move || {
        if open.get() { "open" } else { "closed" }
    };
    view! {
        <div
            data-rs-confirm-dialog-content=""
            data-rs-state=computed_state
            role="alertdialog"
            aria-modal="true"
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ConfirmDialogFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-confirm-dialog-footer="" class=class>
            {children()}
        </div>
    }
}
