//! Sheet Component - Drawer lateral

use leptos::prelude::*;
use crate::primitives::{SheetPrimitive, SheetTriggerPrimitive, SheetContentPrimitive, SheetOverlayPrimitive};

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
    #[prop(into)] target_sheet_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SheetTriggerPrimitive target_sheet_id=target_sheet_id class=class id=id>
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
