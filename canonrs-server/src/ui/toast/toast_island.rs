//! @canon-level: strict
//! Toast Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::toast_ui::{Toast, ToastViewport};
use canonrs_core::primitives::ToastVariant;

#[allow(unused_variables)]
#[component]
pub fn ToastIsland(
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = ToastVariant::Default)] variant: ToastVariant,
    #[prop(default = 5000u32)] duration_ms: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Toast variant=variant class=class>
            {title.map(|t| view! { <p data-rs-toast-title="">{t}</p> })}
            {description.map(|d| view! { <p data-rs-toast-description="">{d}</p> })}
        </Toast>
    }
}

#[component]
pub fn ToastViewportIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ToastViewport class=class>{children()}</ToastViewport> }
}
