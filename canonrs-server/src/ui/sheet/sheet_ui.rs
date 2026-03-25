//! @canon-level: ui
//! Sheet - attribute-driven
//! CONTRACT: SheetContent requer aria_labelledby obrigatorio

use leptos::prelude::*;
use canonrs_core::primitives::{SheetPrimitive, SheetOverlayPrimitive, SheetContentPrimitive, SheetSide};

#[component]
pub fn Sheet(
    children: Children,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SheetPrimitive side=side open=open.into() class=class>
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

/// CONTRATO: aria_labelledby obrigatorio para acessibilidade enterprise
#[component]
pub fn SheetContent(
    children: Children,
    /// ID do titulo do sheet — OBRIGATORIO para ARIA
    #[prop(into)] aria_labelledby: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_describedby: Option<String>,
) -> impl IntoView {
    view! {
        <SheetContentPrimitive
            class=class
            attr:aria-labelledby=aria_labelledby
            attr:aria-describedby=aria_describedby.unwrap_or_default()
        >
            {children()}
        </SheetContentPrimitive>
    }
}

#[component]
pub fn SheetPreview() -> impl IntoView {
    view! {
        <Sheet>
            <button type="button" data-rs-sheet-trigger="">"Open Sheet"</button>
            <SheetOverlay />
            <SheetContent aria_labelledby="sheet-title-preview">
                <h2 id="sheet-title-preview">"Sheet Title"</h2>
                <p>"Sheet content"</p>
                <button type="button" data-rs-sheet-close="">"Close"</button>
            </SheetContent>
        </Sheet>
    }
}
