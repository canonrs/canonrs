//! @canon-level: strict
//! @canon-owner: primitives-team
//! Resizable Primitives - Split panels with draggable divider

use leptos::prelude::*;
use crate::meta::ActivityState;
use std::sync::atomic::{AtomicU32, Ordering};
#[allow(dead_code)]
static RESIZABLE_UID: AtomicU32 = AtomicU32::new(0);
static RESIZABLE_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical   => "vertical",
        }
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
    view! {
        <div
            data-rs-resizable=""
            data-rs-uid=RESIZABLE_ID.fetch_add(1, Ordering::SeqCst).to_string()
            data-rs-interaction="gesture"
            data-rs-orientation=orientation.as_str()
            data-rs-min-size=min_size.to_string()
            data-rs-max-size=max_size.to_string()
            role="group"
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
) -> impl IntoView {
    view! {
        <div
            data-rs-resizable-panel=""
            data-rs-default-size=default_size.to_string()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ResizableHandlePrimitive(
    children: Children,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-resizable-handle=""
            data-rs-activity=state.as_str()
            role="separator"
            tabindex="0"
            class=class
        >
            <div data-rs-resizable-handle-bar="" />
            {children()}
        </div>
    }
}
