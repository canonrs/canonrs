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
            data-rs-uid=crate::infra::uid::generate("sp")
            class=class
        />
    }
}
