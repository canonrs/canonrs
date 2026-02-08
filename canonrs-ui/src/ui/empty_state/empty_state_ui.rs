use leptos::prelude::*;
use crate::primitives::{
    EmptyStatePrimitive,
    EmptyStateIconPrimitive,
    EmptyStateTitlePrimitive,
    EmptyStateDescriptionPrimitive,
    EmptyStateActionPrimitive,
};

#[derive(Clone, Copy, PartialEq)]
pub enum EmptyStateVariant {
    Default,
    NoData,
    NoResults,
    Error,
}

impl EmptyStateVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmptyStateVariant::Default => "default",
            EmptyStateVariant::NoData => "no-data",
            EmptyStateVariant::NoResults => "no-results",
            EmptyStateVariant::Error => "error",
        }
    }
}

#[component]
pub fn EmptyState(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("empty-state variant-{} {}", variant.as_str(), class);

    view! {
        <EmptyStatePrimitive class={base_class} id={id}>
            {children.map(|c| c())}
        </EmptyStatePrimitive>
    }
}

#[component]
pub fn EmptyStateIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateIconPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </EmptyStateIconPrimitive>
    }
}

#[component]
pub fn EmptyStateTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateTitlePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </EmptyStateTitlePrimitive>
    }
}

#[component]
pub fn EmptyStateDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateDescriptionPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </EmptyStateDescriptionPrimitive>
    }
}

#[component]
pub fn EmptyStateAction(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateActionPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </EmptyStateActionPrimitive>
    }
}
