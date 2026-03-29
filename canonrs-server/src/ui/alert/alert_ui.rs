//! @canon-id: alert
//! @canon-label: Alert
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Display important static messages
//! @canon-description: Alert message box
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: AlertTitle, AlertDescription
//! @canon-tags: alert, warning, info, message, error

use leptos::prelude::*;
use canonrs_core::primitives::{
    AlertPrimitive, AlertTitlePrimitive, AlertDescriptionPrimitive,
    AlertCloseButtonPrimitive, AlertVariant,
};

#[component]
pub fn Alert(
    children: Children,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertPrimitive variant=variant class=class>
            {children()}
        </AlertPrimitive>
    }
}

#[component]
pub fn AlertTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertTitlePrimitive class=class>
            {children()}
        </AlertTitlePrimitive>
    }
}

#[component]
pub fn AlertDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertDescriptionPrimitive class=class>
            {children()}
        </AlertDescriptionPrimitive>
    }
}

#[component]
pub fn AlertCloseButton(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertCloseButtonPrimitive class=class>
            {children()}
        </AlertCloseButtonPrimitive>
    }
}

#[component]
pub fn AlertPreview() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Default>
            <AlertTitle>"Info"</AlertTitle>
            <AlertDescription>"This is a default alert message."</AlertDescription>
        </Alert>
    }
}
