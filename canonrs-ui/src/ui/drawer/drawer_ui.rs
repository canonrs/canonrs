use leptos::prelude::*;
use crate::primitives::{DrawerPrimitive, DrawerTriggerPrimitive, DrawerContentPrimitive, DrawerOverlayPrimitive};

#[component]
pub fn Drawer(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerPrimitive
            class=class
            id=id
        >
            {children()}
        </DrawerPrimitive>
    }
}

#[component]
pub fn DrawerTrigger(
    children: Children,
    #[prop(into)] target_drawer_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerTriggerPrimitive
            target_drawer_id=target_drawer_id
            class=class
            id=id
        >
            {children()}
        </DrawerTriggerPrimitive>
    }
}

#[component]
pub fn DrawerContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerContentPrimitive
            class=class
            id=id
        >
            {children()}
        </DrawerContentPrimitive>
    }
}

#[component]
pub fn DrawerOverlay(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DrawerOverlayPrimitive
            class=class
            id=id
        />
    }
}
