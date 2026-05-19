#![allow(unreachable_pub, dead_code)]

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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CalloutPrimitive variant=variant class=class>
            {children()}
        </CalloutPrimitive>
    }
}

#[component]
pub fn CalloutIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CalloutIconPrimitive class=class>
            {children()}
        </CalloutIconPrimitive>
    }
}

#[component]
pub fn CalloutTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CalloutTitlePrimitive class=class>
            {children()}
        </CalloutTitlePrimitive>
    }
}

#[component]
pub fn CalloutDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CalloutDescriptionPrimitive class=class>
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
