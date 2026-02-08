use leptos::prelude::*;
use crate::primitives::{
    CalloutPrimitive,
    CalloutIconPrimitive,
    CalloutTitlePrimitive,
    CalloutDescriptionPrimitive,
};

#[derive(Clone, Copy, PartialEq)]
pub enum CalloutVariant {
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl CalloutVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            CalloutVariant::Default => "default",
            CalloutVariant::Info => "info",
            CalloutVariant::Success => "success",
            CalloutVariant::Warning => "warning",
            CalloutVariant::Error => "error",
        }
    }
}

#[component]
pub fn Callout(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CalloutPrimitive
            attr:data-variant={variant.as_str()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CalloutPrimitive>
    }
}

#[component]
pub fn CalloutIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CalloutIconPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </CalloutIconPrimitive>
    }
}

#[component]
pub fn CalloutTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CalloutTitlePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </CalloutTitlePrimitive>
    }
}

#[component]
pub fn CalloutDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CalloutDescriptionPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </CalloutDescriptionPrimitive>
    }
}
