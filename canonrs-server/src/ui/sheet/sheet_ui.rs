#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{SheetPrimitive, SheetTriggerPrimitive, SheetOverlayPrimitive, SheetContentPrimitive, SheetSide};
use canonrs_core::primitives::SheetPortalPrimitive;
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Sheet(
    children: Children,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SheetPrimitive side=side state=state class=class>
            {children()}
        </SheetPrimitive>
    }
}

#[component]
pub fn SheetOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SheetOverlayPrimitive class=class /> }
}

#[component]
pub fn SheetContent(
    children: Children,
    #[prop(into)] aria_labelledby: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_describedby: Option<String>,
) -> impl IntoView {
    view! {
        <SheetContentPrimitive
            class=class
            aria_labelledby=aria_labelledby
            aria_describedby=aria_describedby.unwrap_or_default()
        >
            {children()}
        </SheetContentPrimitive>
    }
}


#[component]
pub fn SheetTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SheetTriggerPrimitive class=class>{children()}</SheetTriggerPrimitive> }
}

#[component]
pub fn SheetPortal(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SheetPortalPrimitive class=class>{children()}</SheetPortalPrimitive> }
}
