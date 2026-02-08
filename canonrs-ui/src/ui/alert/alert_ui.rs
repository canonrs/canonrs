use leptos::prelude::*;
use crate::primitives::{AlertPrimitive, AlertTitlePrimitive, AlertDescriptionPrimitive, AlertVariant};

#[component]
pub fn Alert(
    children: Children,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AlertPrimitive variant={variant} class={class.unwrap_or_default()} id={id.unwrap_or_default()}>
            {children()}
        </AlertPrimitive>
    }
}

#[component]
pub fn AlertTitle(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AlertTitlePrimitive class={class.unwrap_or_default()} id={id.unwrap_or_default()}>
            {children()}
        </AlertTitlePrimitive>
    }
}

#[component]
pub fn AlertDescription(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AlertDescriptionPrimitive class={class.unwrap_or_default()} id={id.unwrap_or_default()}>
            {children()}
        </AlertDescriptionPrimitive>
    }
}

#[component]
pub fn AlertCloseButton(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            attr:data-alert-close=""
            type="button"
            attr:aria-label="Close alert"
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </button>
    }
}
