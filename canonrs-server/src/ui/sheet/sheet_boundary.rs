//! @canon-level: strict
//! Sheet Island — Canon Rule #340 (zero-logic boundary)
//! CR-342 v3.0.0: interaction delegated to canonrs-interactions-overlay

use leptos::prelude::*;
use super::sheet_ui::{
    Sheet as SheetUi,
    SheetTrigger,
    SheetOverlay as SheetOverlayUi,
    SheetContent as SheetContentUi,
    SheetPortal
};
pub use canonrs_core::primitives::SheetSide;
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Sheet(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::from("Open"))] trigger_label: String,
    #[prop(into, default = String::from("Close"))] close_label: String,
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SheetUi side=side state=VisibilityState::Closed class=class>
            {if !trigger_label.is_empty() { Some(view! { <SheetTrigger>{trigger_label}</SheetTrigger> }) } else { None }}
            <SheetPortal>
            <SheetOverlayUi />
            <SheetContentUi aria_labelledby="sheet-title">
                <h2 data-rs-sheet-title="">{title.unwrap_or_default()}</h2>
                <p data-rs-sheet-description="">{description.unwrap_or_default()}</p>
                {children.map(|c| c())}
                <button type="button" data-rs-sheet-close="">
                    {close_label}
                </button>
            </SheetContentUi>
            </SheetPortal>
        </SheetUi>
    }
}

#[component]
pub fn SheetOverlay(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SheetOverlayUi class=class /> }
}

#[component]
pub fn SheetContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] aria_labelledby: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] aria_describedby: String,
) -> impl IntoView {
    view! {
        <SheetContentUi
            aria_labelledby=aria_labelledby
            aria_describedby=aria_describedby
            class=class
        >
            {children.map(|c| c())}
        </SheetContentUi>
    }
}
