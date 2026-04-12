#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    DrawerPrimitive, DrawerTriggerPrimitive, DrawerOverlayPrimitive,
    DrawerContentPrimitive, DrawerSide,
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Drawer(
    children: Children,
    #[prop(default = DrawerSide::Right)] side: DrawerSide,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DrawerPrimitive side=side state=state class=class>{children()}</DrawerPrimitive> }
}

#[component]
pub fn DrawerTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DrawerTriggerPrimitive class=class>{children()}</DrawerTriggerPrimitive> }
}

#[component]
pub fn DrawerOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DrawerOverlayPrimitive class=class /> }
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] aria_labelledby: String,
) -> impl IntoView {
    view! {
        <DrawerContentPrimitive aria_labelledby=aria_labelledby class=class>
            {children()}
        </DrawerContentPrimitive>
    }
}
