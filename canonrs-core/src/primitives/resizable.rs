//! @canon-level: strict
//! @canon-owner: primitives-team
//! Resizable Primitives - Split panels with draggable divider

use leptos::prelude::*;

#[component]
pub fn ResizablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(into, default = "horizontal".to_string())] orientation: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-resizable=""
            data-rs-orientation=orientation
            role="group"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ResizablePanelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-resizable-panel=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ResizableHandlePrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(into, default = "horizontal".to_string())] orientation: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-resizable-handle=""
            data-rs-state="inactive"
            role="separator"
            aria-orientation=orientation
            tabindex="0"
            class=class
            id=id
        >
            <div data-rs-resizable-handle-bar="" />
        </div>
    }
}
