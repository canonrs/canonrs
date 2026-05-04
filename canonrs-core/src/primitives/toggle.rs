//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toggle Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ToggleState, DisabledState};


#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = ToggleState::Off)] pressed: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_tog = crate::infra::uid::generate("tog");
    view! {
        <label
            data-rs-toggle=""
            data-rs-uid=uid_tog
            data-rs-interaction="init"
            data-rs-pressed=pressed.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-label=if aria_label.is_empty() { None } else { Some(aria_label) }
            aria-disabled=disabled.aria_disabled()
            class=class
        >
            <input
                type="checkbox"
                data-rs-toggle-input=""
                checked=pressed.as_str() == "on"
                tabindex="-1"
            />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
