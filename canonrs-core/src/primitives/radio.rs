//! @canon-level: strict
//! @canon-owner: primitives-team
//! Radio Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_ra = crate::infra::uid::generate("ra");
    let is_selected = selected == SelectionState::Selected;
    let is_disabled = disabled == DisabledState::Disabled;

    view! {
        <label
            data-rs-radio=""
            data-rs-uid=uid_ra
            data-rs-interaction="init"
            data-rs-selection=if is_selected { Some("selected") } else { None }
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            class=class
        >
            <input
                type="radio"
                data-rs-radio-input=""
                name=name
                value=value
                checked=is_selected
                disabled=is_disabled
                aria-checked=if is_selected { "true" } else { "false" }
                aria-disabled=if is_disabled { Some("true") } else { None }
            />
            <span data-rs-radio-indicator="" />
            {children()}
        </label>
    }
}
