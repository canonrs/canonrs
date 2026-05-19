#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{ResizablePrimitive, ResizablePanelPrimitive, ResizableHandlePrimitive, ResizableOrientation};

#[component]
pub fn Resizable(
    children: Children,
    #[prop(default = ResizableOrientation::Horizontal)] orientation: ResizableOrientation,
    #[prop(default = 20u32)] min_size: u32,
    #[prop(default = 80u32)] max_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ResizablePrimitive orientation=orientation min_size=min_size max_size=max_size class=class>{children()}</ResizablePrimitive> }
}

#[component]
pub fn ResizablePanel(
    children: Children,
    #[prop(default = 50u32)] default_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ResizablePanelPrimitive default_size=default_size class=class>{children()}</ResizablePanelPrimitive> }
}

#[component]
pub fn ResizableHandle(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ResizableHandlePrimitive class=class /> }
}
