//! @canon-level: strict
//! @canon-owner: primitives-team
//! Radio Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};
use crate::infra::state_engine::{disabled_attrs, selection_attrs};

#[component]
pub fn RadioPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sel = selection_attrs(selected);
    let d = disabled_attrs(disabled);

    // Combina selected + disabled no data-rs-state para SSR funcionar
    let state_str: Option<String> = match (sel.data_rs_state, disabled == DisabledState::Disabled) {
        (Some(s), true)  => Some(format!("{} disabled", s)),
        (Some(s), false) => Some(s.to_string()),
        (None,    true)  => Some("disabled".to_string()),
        (None,    false) => None,
    };

    view! {
        <label
            data-rs-radio=""
            data-rs-interaction="selection"
            data-rs-component="Radio"
            data-rs-behavior="radio"
            data-rs-state=state_str
            data-rs-disabled=d.data_rs_disabled
            class=class
        >
            <input
                type="radio"
                data-rs-radio-input=""
                name=name
                value=value
                checked=sel.data_rs_state == Some("selected")
                disabled=d.data_rs_disabled.is_some()
                aria-checked=sel.aria_selected
                aria-disabled=d.aria_disabled
            />
            <span data-rs-radio-indicator="" />
            {children()}
        </label>
    }
}
