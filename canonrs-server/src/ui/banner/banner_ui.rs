//! @canon-level: ui
//! Banner - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::{
    BannerPrimitive, BannerClosePrimitive,
    BannerContentPrimitive, BannerActionsPrimitive,
};
pub use canonrs_core::primitives::BannerVariant;

#[component]
pub fn Banner(
    children: Children,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BannerPrimitive variant=variant class={class.unwrap_or_default()}>
            {children()}
        </BannerPrimitive>
    }
}

#[component]
pub fn BannerContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BannerContentPrimitive class={class.unwrap_or_default()}>
            {children()}
        </BannerContentPrimitive>
    }
}

#[component]
pub fn BannerActions(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BannerActionsPrimitive class={class.unwrap_or_default()}>
            {children()}
        </BannerActionsPrimitive>
    }
}

#[component]
pub fn BannerClose(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <BannerClosePrimitive class={class.unwrap_or_default()}>
            {children()}
        </BannerClosePrimitive>
    }
}

#[component]
pub fn BannerPreview() -> impl IntoView {
    view! {
        <Banner variant=BannerVariant::Info>
            <BannerContent>"System maintenance scheduled for Saturday."</BannerContent>
            <BannerClose>"×"</BannerClose>
        </Banner>
    }
}
