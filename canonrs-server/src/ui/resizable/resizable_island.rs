//! Resizable Island — Canon Rule passthrough
use leptos::prelude::*;
use super::resizable_ui::{Resizable, ResizablePanel, ResizableHandle};
use canonrs_core::primitives::ResizableOrientation;

#[component]
pub fn ResizableIsland(
    children: Children,
    #[prop(default = ResizableOrientation::Horizontal)] orientation: ResizableOrientation,
    #[prop(default = 20u32)] min_size: u32,
    #[prop(default = 80u32)] max_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Resizable orientation=orientation min_size=min_size max_size=max_size class=class>
            {children()}
        </Resizable>
    }
}

#[component]
pub fn ResizablePanelIsland(
    children: Children,
    #[prop(default = 50u32)] default_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ResizablePanel default_size=default_size class=class>
            {children()}
        </ResizablePanel>
    }
}

#[component]
pub fn ResizableHandleIsland(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ResizableHandle class=class /> }
}
