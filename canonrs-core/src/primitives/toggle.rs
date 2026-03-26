//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toggle Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ToggleState, DisabledState};
use crate::state_engine::disabled_attrs;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = ToggleState::Off)] pressed: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <label
            data-rs-toggle=""
            data-rs-component="Toggle"
            data-rs-behavior="toggle"
            data-rs-state=pressed.as_str()
            data-rs-disabled=d.data_rs_disabled
            aria-label=if aria_label.is_empty() { None } else { Some(aria_label) }
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                data-rs-toggle-input=""
                aria-hidden="true"
                tabindex="-1"
            />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
