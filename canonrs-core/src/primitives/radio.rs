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
    view! {
        <label
            data-rs-radio=""
            data-rs-component="Radio"
            data-rs-behavior="selection"
            data-rs-state=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            class=class
        >
            <input
                type="radio"
                data-rs-radio-input=""
                name=name
                value=value
                aria-selected=sel.aria_selected
                aria-disabled=d.aria_disabled
            />
            <span data-rs-radio-indicator="" />
            {children()}
        </label>
    }
}
