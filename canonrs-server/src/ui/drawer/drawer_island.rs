//! @canon-level: strict
//! Drawer Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::drawer_ui::{Drawer, DrawerTrigger, DrawerOverlay, DrawerContent};
use canonrs_core::primitives::DrawerSide;
use canonrs_core::meta::VisibilityState;

#[component]
pub fn DrawerIsland(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::from("Open"))] trigger_label: String,
    #[prop(into, default = String::from("Close"))] close_label: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = DrawerSide::Right)] side: DrawerSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Drawer side=side state=VisibilityState::Closed class=class>
            <DrawerTrigger>{trigger_label}</DrawerTrigger>
            <DrawerOverlay />
            <DrawerContent aria_labelledby="drawer-title">
                {title.map(|t| view! { <h2 id="drawer-title" data-rs-drawer-title="">{t}</h2> })}
                {description.map(|d| view! { <p data-rs-drawer-description="">{d}</p> })}
                {children.map(|c| c())}
                <button type="button" data-rs-drawer-close="">{close_label}</button>
            </DrawerContent>
        </Drawer>
    }
}
