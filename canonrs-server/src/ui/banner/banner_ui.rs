
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BannerPrimitive variant=variant class=class>
            {children()}
        </BannerPrimitive>
    }
}

#[component]
pub fn BannerContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BannerContentPrimitive class=class>
            {children()}
        </BannerContentPrimitive>
    }
}

#[component]
pub fn BannerActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BannerActionsPrimitive class=class>
            {children()}
        </BannerActionsPrimitive>
    }
}

#[component]
pub fn BannerClose(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BannerClosePrimitive class=class>
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
