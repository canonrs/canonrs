//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toggle Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-toggle=""
            data-rs-state={if pressed { "on" } else { "off" }}
            aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}
            class=class
        >
            <input
                type="checkbox"
                prop:checked=pressed
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
