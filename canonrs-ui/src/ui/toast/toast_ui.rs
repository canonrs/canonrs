use leptos::prelude::*;
use crate::primitives::{
    ToastPrimitive,
    ToastTitlePrimitive,
    ToastDescriptionPrimitive,
    ToastClosePrimitive,
};

pub use crate::primitives::ToastVariant;

#[component]
pub fn Toast(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(default = true)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ToastPrimitive variant=variant open=open class=class id=id>
            {children.map(|c| c())}
        </ToastPrimitive>
    }
}

#[component]
pub fn ToastTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ToastTitlePrimitive class=class id=id>
            {children.map(|c| c())}
        </ToastTitlePrimitive>
    }
}

#[component]
pub fn ToastDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ToastDescriptionPrimitive class=class id=id>
            {children.map(|c| c())}
        </ToastDescriptionPrimitive>
    }
}

#[component]
pub fn ToastClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <ToastClosePrimitive class=class id=id>
            {children.map(|c| c())}
        </ToastClosePrimitive>
    }
}
