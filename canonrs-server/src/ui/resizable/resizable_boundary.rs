//! Resizable Island — Canon Rule passthrough
use leptos::prelude::*;
use super::resizable_ui::{
    Resizable as ResizableUi,
    ResizablePanel as ResizablePanelUi,
    ResizableHandle as ResizableHandleUi
};
pub use canonrs_core::primitives::ResizableOrientation;

#[component]
pub fn Resizable(
    children: Children,
    #[prop(default = ResizableOrientation::Horizontal)] orientation: ResizableOrientation,
    #[prop(default = 20u32)] min_size: u32,
    #[prop(default = 80u32)] max_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ResizableUi orientation=orientation min_size=min_size max_size=max_size class=class>
            {children()}
        </ResizableUi>
    }
}

#[component]
pub fn ResizablePanel(
    children: Children,
    #[prop(default = 50u32)] default_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ResizablePanelUi default_size=default_size class=class>
            {children()}
        </ResizablePanelUi>
    }
}

#[component]
pub fn ResizableHandle(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ResizableHandleUi class=class /> }
}
