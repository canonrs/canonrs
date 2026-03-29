//! @canon-level: strict
//! @canon-owner: primitives-team
//! Hero Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn HeroPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="Hero"
            data-rs-behavior="structural"
            class=class
        >
            {children()}
        </div>
    }
}
