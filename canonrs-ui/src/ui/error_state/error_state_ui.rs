use leptos::prelude::*;
use crate::primitives::{
    ErrorStatePrimitive,
    ErrorStateIconPrimitive,
    ErrorStateTitlePrimitive,
    ErrorStateDescriptionPrimitive,
    ErrorStateActionsPrimitive,
};

#[component]
pub fn ErrorState(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ErrorStatePrimitive class=class id=id>
            {children.map(|c| c())}
        </ErrorStatePrimitive>
    }
}

#[component]
pub fn ErrorStateIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ErrorStateIconPrimitive class=class id=id>
            {children.map(|c| c())}
        </ErrorStateIconPrimitive>
    }
}

#[component]
pub fn ErrorStateTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ErrorStateTitlePrimitive class=class id=id>
            {children.map(|c| c())}
        </ErrorStateTitlePrimitive>
    }
}

#[component]
pub fn ErrorStateDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ErrorStateDescriptionPrimitive class=class id=id>
            {children.map(|c| c())}
        </ErrorStateDescriptionPrimitive>
    }
}

#[component]
pub fn ErrorStateActions(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ErrorStateActionsPrimitive class=class id=id>
            {children.map(|c| c())}
        </ErrorStateActionsPrimitive>
    }
}
