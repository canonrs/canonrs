//! @canon-level: strict
//! Resizable Primitives
use leptos::prelude::*;
use crate::infra::uid::generate;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum ResizableOrientation { #[default] Horizontal, Vertical }
impl ResizableOrientation {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Horizontal => "horizontal", Self::Vertical => "vertical" }
    }
}

#[component]
pub fn ResizablePrimitive(
    children: Children,
    #[prop(default = ResizableOrientation::Horizontal)] orientation: ResizableOrientation,
    #[prop(default = 20u32)] min_size: u32,
    #[prop(default = 80u32)] max_size: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("rs");
    view! {
        <div
            data-rs-resizable=""
            data-rs-uid=uid
            data-rs-interaction="gesture"
            data-rs-orientation=orientation.as_str()
            data-rs-min-size=min_size.to_string()
            data-rs-max-size=max_size.to_string()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ResizablePanelPrimitive(
    children: Children,
    #[prop(default = 50u32)] default_size: u32,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let uid = generate("rsp");
    view! {
        <div
            data-rs-resizable-panel=""
            data-rs-uid=uid
            data-rs-default-size=default_size.to_string()
            id=id
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ResizableHandlePrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let uid = generate("rsh");
    view! {
        <div
            data-rs-resizable-handle=""
            data-rs-uid=uid
            data-rs-disabled=disabled.then_some("disabled")
            aria-disabled=if disabled { Some("true") } else { None }
            id=id
            class=class
        >
            <span data-rs-resizable-handle-icon="" aria-hidden="true" />
        </div>
    }
}
