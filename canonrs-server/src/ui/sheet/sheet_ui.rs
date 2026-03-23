//! Sheet Component - Drawer lateral

use leptos::prelude::*;
use canonrs_core::primitives::{SheetPrimitive, SheetTriggerPrimitive, SheetContentPrimitive, SheetOverlayPrimitive};

#[derive(Clone, Copy, PartialEq)]
pub enum SheetSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl SheetSide {
    pub fn as_str(&self) -> &str {
        match self {
            SheetSide::Left => "left",
            SheetSide::Right => "right",
            SheetSide::Top => "top",
            SheetSide::Bottom => "bottom",
        }
    }
}

#[component]
pub fn Sheet(
    children: Children,
    #[prop(into)] id: String,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SheetPrimitive id=id side={side.as_str().to_string()} class=class>
            {children()}
        </SheetPrimitive>
    }
}

#[component]
pub fn SheetTrigger(
    children: Children,
        #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SheetTriggerPrimitive class=class id=id>
            {children()}
        </SheetTriggerPrimitive>
    }
}

#[component]
pub fn SheetContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SheetContentPrimitive class=class id=id>
            {children()}
        </SheetContentPrimitive>
    }
}

#[component]
pub fn SheetOverlay(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SheetOverlayPrimitive class=class id=id />
    }
}

#[component]
pub fn SheetPreview() -> impl IntoView {
    view! { <Sheet id="sheet-preview".to_string()>"Content"</Sheet> }
}
