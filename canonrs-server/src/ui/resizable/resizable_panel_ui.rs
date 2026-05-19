#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::ResizablePanelPrimitive;

#[component]
pub fn ResizablePanel(
    children: Children,
    #[prop(default = 50u32)] default_size: u32,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! { <ResizablePanelPrimitive default_size=default_size class=class id=id>{children()}</ResizablePanelPrimitive> }
}
