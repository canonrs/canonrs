use leptos::prelude::*;
use crate::primitives::{
    BannerPrimitive,
    BannerContentPrimitive,
    BannerActionsPrimitive,
    BannerClosePrimitive,
};

#[derive(Clone, Copy, PartialEq)]
pub enum BannerVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl BannerVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            BannerVariant::Info => "info",
            BannerVariant::Success => "success",
            BannerVariant::Warning => "warning",
            BannerVariant::Error => "error",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BannerPosition {
    Top,
    Bottom,
}

impl BannerPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            BannerPosition::Top => "top",
            BannerPosition::Bottom => "bottom",
        }
    }
}

#[component]
pub fn Banner(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = BannerPosition::Top)] position: BannerPosition,
    #[prop(default = true)] open: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!(
        "banner variant-{} position-{} {}",
        variant.as_str(),
        position.as_str(),
        class
    );

    view! {
        <BannerPrimitive open={open} class={base_class} id={id}>
            {children.map(|c| c())}
        </BannerPrimitive>
    }
}

#[component]
pub fn BannerContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <BannerContentPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </BannerContentPrimitive>
    }
}

#[component]
pub fn BannerActions(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <BannerActionsPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </BannerActionsPrimitive>
    }
}

#[component]
pub fn BannerClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <BannerClosePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </BannerClosePrimitive>
    }
}
