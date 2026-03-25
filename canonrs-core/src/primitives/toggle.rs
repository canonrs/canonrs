//! @canon-level: strict
//! Toggle Primitive - native checkbox + CSS

use leptos::prelude::*;
use crate::meta::ToggleState;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] pressed: ToggleState,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-toggle=""
            data-rs-component="Toggle"
            data-rs-behavior="toggle"
            data-rs-state=pressed.as_str()
            aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}
            class=class
        >
            <input type="checkbox" prop:checked={pressed == ToggleState::On} />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
