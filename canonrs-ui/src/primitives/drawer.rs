//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drawer Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn DrawerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-drawer="" data-state="closed" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_drawer_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-drawer-trigger={target_drawer_id} class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-drawer-content="" role="dialog" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-drawer-overlay="" aria-hidden="true" class=class id=id />
    }
}
