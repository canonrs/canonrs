use leptos::prelude::*;
use crate::primitives::{
    InlineNoticePrimitive,
    InlineNoticeIconPrimitive,
    InlineNoticeContentPrimitive,
};

#[derive(Clone, Copy, PartialEq)]
pub enum InlineNoticeVariant {
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl InlineNoticeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            InlineNoticeVariant::Default => "default",
            InlineNoticeVariant::Info => "info",
            InlineNoticeVariant::Success => "success",
            InlineNoticeVariant::Warning => "warning",
            InlineNoticeVariant::Error => "error",
        }
    }
}

#[component]
pub fn InlineNotice(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("inline-notice variant-{} {}", variant.as_str(), class);

    view! {
        <InlineNoticePrimitive class={base_class} id={id}>
            {children.map(|c| c())}
        </InlineNoticePrimitive>
    }
}

#[component]
pub fn InlineNoticeIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <InlineNoticeIconPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </InlineNoticeIconPrimitive>
    }
}

#[component]
pub fn InlineNoticeContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <InlineNoticeContentPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </InlineNoticeContentPrimitive>
    }
}
