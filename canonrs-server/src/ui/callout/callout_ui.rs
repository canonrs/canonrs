//! @canon-id: callout
//! @canon-label: Callout
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Highlight important contextual information
//! @canon-description: Callout info box
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: CalloutIcon, CalloutTitle, CalloutDescription
//! @canon-tags: callout, highlight, info, note, warning, tip

use leptos::prelude::*;
use canonrs_core::primitives::{
    CalloutPrimitive, CalloutIconPrimitive,
    CalloutTitlePrimitive, CalloutDescriptionPrimitive,
};
pub use canonrs_core::primitives::CalloutVariant;

#[component]
pub fn Callout(
    children: Children,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CalloutPrimitive variant=variant class=class.unwrap_or_default()>
            {children()}
        </CalloutPrimitive>
    }
}

#[component]
pub fn CalloutIcon(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CalloutIconPrimitive class={class.unwrap_or_default()}>
            {children()}
        </CalloutIconPrimitive>
    }
}

#[component]
pub fn CalloutTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CalloutTitlePrimitive class={class.unwrap_or_default()}>
            {children()}
        </CalloutTitlePrimitive>
    }
}

#[component]
pub fn CalloutDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <CalloutDescriptionPrimitive class={class.unwrap_or_default()}>
            {children()}
        </CalloutDescriptionPrimitive>
    }
}

#[component]
pub fn CalloutPreview() -> impl IntoView {
    view! {
        <Callout variant=CalloutVariant::Info>
            <CalloutTitle>"Information"</CalloutTitle>
            <CalloutDescription>"New features are available in the latest release."</CalloutDescription>
        </Callout>
    }
}
