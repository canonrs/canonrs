use leptos::prelude::*;
use crate::primitives::{
    EmptyStatePrimitive,
    EmptyStateIconPrimitive,
    EmptyStateTitlePrimitive,
    EmptyStateDescriptionPrimitive,
    EmptyStateActionPrimitive,
};

pub use crate::primitives::EmptyStateVariant;

#[component]
pub fn EmptyState(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyStatePrimitive variant=variant class=class id=id>
            {children.map(|c| c())}
        </EmptyStatePrimitive>
    }
}

#[component]
pub fn EmptyStateIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateIconPrimitive class=class id=id>
            {children.map(|c| c())}
        </EmptyStateIconPrimitive>
    }
}

#[component]
pub fn EmptyStateTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateTitlePrimitive class=class id=id>
            {children.map(|c| c())}
        </EmptyStateTitlePrimitive>
    }
}

#[component]
pub fn EmptyStateDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateDescriptionPrimitive class=class id=id>
            {children.map(|c| c())}
        </EmptyStateDescriptionPrimitive>
    }
}

#[component]
pub fn EmptyStateAction(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <EmptyStateActionPrimitive class=class id=id>
            {children.map(|c| c())}
        </EmptyStateActionPrimitive>
    }
}
