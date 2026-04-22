//! @canon-level: strict
//! Toast Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::toast_ui::{
    Toast as ToastUi,
    ToastViewport as ToastViewportUi
};
pub use canonrs_core::primitives::ToastVariant;

#[allow(unused_variables)]
#[component]
pub fn Toast(
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(default = 5000u32)] duration_ms: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToastUi variant=variant class=class>
            {title.map(|t| view! { <p data-rs-toast-title="">{t}</p> })}
            {description.map(|d| view! { <p data-rs-toast-description="">{d}</p> })}
        </ToastUi>
    }
}

#[component]
pub fn ToastViewport(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ToastViewportUi class=class>{children()}</ToastViewportUi> }
}
