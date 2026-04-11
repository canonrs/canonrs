//! @canon-level: strict
//! Sheet Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::sheet_ui::{Sheet, SheetTrigger, SheetOverlay, SheetContent};
use canonrs_core::primitives::SheetSide;
use canonrs_core::meta::VisibilityState;

#[component]
pub fn SheetIsland(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::from("Open"))] trigger_label: String,
    #[prop(into, default = String::from("Close"))] close_label: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Sheet side=side state=VisibilityState::Closed class=class>
            {if !trigger_label.is_empty() { Some(view! { <SheetTrigger>{trigger_label}</SheetTrigger> }) } else { None }}
            <SheetOverlay />
            <SheetContent aria_labelledby="sheet-title">
                <h2 data-rs-sheet-title="">{title.unwrap_or_default()}</h2>
                <p data-rs-sheet-description="">{description.unwrap_or_default()}</p>
                {children.map(|c| c())}
                <button type="button" data-rs-sheet-close="">
                    {close_label}
                </button>
            </SheetContent>
        </Sheet>
    }
}

#[component]
pub fn SheetOverlayIsland(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SheetOverlay class=class /> }
}

#[component]
pub fn SheetContentIsland(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] aria_labelledby: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] aria_describedby: String,
) -> impl IntoView {
    view! {
        <SheetContent
            aria_labelledby=aria_labelledby
            aria_describedby=aria_describedby
            class=class
        >
            {children.map(|c| c())}
        </SheetContent>
    }
}
