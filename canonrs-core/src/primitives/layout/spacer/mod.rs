//! @canon-level: strict
//! @canon-owner: primitives-team
//! Spacer Primitive - Auto-expanding spacer for flex layouts

use leptos::prelude::*;

#[component]
pub fn SpacerPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-spacer=""
            class=class
        />
    }
}

/// Construtor funcional — retorna AnyView sem passar pelo macro view!
pub fn spacer_view() -> AnyView {
    view! {
        <div
            data-rs-spacer=""
        />
    }.into_any()
}
