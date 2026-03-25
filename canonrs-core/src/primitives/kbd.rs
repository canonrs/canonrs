//! @canon-level: strict
//! @canon-owner: primitives-team
//! Kbd Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn KbdPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] size: String,
    #[prop(into, default = String::new())] variant: String,
) -> impl IntoView {
    view! {
        <kbd
            data-rs-kbd=""
            data-rs-size=if size.is_empty() { None } else { Some(size) }
            data-rs-variant=if variant.is_empty() { None } else { Some(variant) }
            class=class
        >
            {children()}
        </kbd>
    }
}
