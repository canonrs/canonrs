//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sheet Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn SheetPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::from("right"))] side: String,
) -> impl IntoView {
    view! {
        <div data-sheet="" data-state="closed" data-side={side} class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SheetTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] target_sheet_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-sheet-trigger={target_sheet_id} class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SheetContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-sheet-content="" role="dialog" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SheetOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-sheet-overlay="" aria-hidden="true" class=class id=id />
    }
}
