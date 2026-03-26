//! @canon-level: ui
//! Toast - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::{
    ToastPrimitive, ToastViewportPrimitive,
    ToastTitlePrimitive, ToastDescriptionPrimitive,
    ToastActionPrimitive, ToastClosePrimitive,
};
pub use canonrs_core::primitives::ToastVariant;

#[component]
pub fn Toast(
    children: Children,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastPrimitive variant=variant class={class.unwrap_or_default()}>
            {children()}
        </ToastPrimitive>
    }
}

#[component]
pub fn ToastViewport(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastViewportPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ToastViewportPrimitive>
    }
}

#[component]
pub fn ToastTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastTitlePrimitive class={class.unwrap_or_default()}>
            {children()}
        </ToastTitlePrimitive>
    }
}

#[component]
pub fn ToastDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastDescriptionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </ToastDescriptionPrimitive>
    }
}

#[component]
pub fn ToastAction(
    children: Children,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastActionPrimitive aria_label=aria_label class={class.unwrap_or_default()}>
            {children()}
        </ToastActionPrimitive>
    }
}

#[component]
pub fn ToastClose(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ToastClosePrimitive class={class.unwrap_or_default()}>
            {children()}
        </ToastClosePrimitive>
    }
}

#[component]
pub fn ToastPreview() -> impl IntoView {
    view! {
        <Toast variant=ToastVariant::Default>
            <ToastTitle>"Notification"</ToastTitle>
            <ToastDescription>"Your settings have been updated."</ToastDescription>
            <ToastClose>"×"</ToastClose>
        </Toast>
    }
}
