//! @canon-level: ui
//! Drawer - attribute-driven
//! CONTRACT: DrawerContent requer aria_labelledby obrigatorio

use leptos::prelude::*;
use canonrs_core::primitives::{DrawerPrimitive, DrawerOverlayPrimitive, DrawerContentPrimitive};

#[component]
pub fn Drawer(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DrawerPrimitive open=open.into() class=class>
            {children()}
        </DrawerPrimitive>
    }
}

#[component]
pub fn DrawerOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DrawerOverlayPrimitive class=class /> }
}

/// CONTRATO: aria_labelledby obrigatorio para acessibilidade enterprise
#[component]
pub fn DrawerContent(
    children: Children,
    /// ID do titulo do drawer — OBRIGATORIO para ARIA
    #[prop(into)] aria_labelledby: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_describedby: Option<String>,
) -> impl IntoView {
    view! {
        <DrawerContentPrimitive
            class=class
            aria_labelledby=aria_labelledby
            aria_describedby=aria_describedby.unwrap_or_default()
        >
            {children()}
        </DrawerContentPrimitive>
    }
}

#[component]
pub fn DrawerPreview() -> impl IntoView {
    view! {
        <Drawer>
            <button type="button" data-rs-drawer-trigger="">"Open Drawer"</button>
            <DrawerOverlay />
            <DrawerContent aria_labelledby="drawer-title-preview">
                <h2 id="drawer-title-preview">"Drawer Title"</h2>
                <p>"Drawer content"</p>
                <button type="button" data-rs-drawer-close="">"Close"</button>
            </DrawerContent>
        </Drawer>
    }
}
