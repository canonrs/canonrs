//! @canon-level: strict
//! @canon-owner: primitives-team
//! Switch Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};
use crate::infra::state_engine::{disabled_attrs, selection_attrs};

#[component]
pub fn SwitchPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] checked: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sel = selection_attrs(checked);
    let d = disabled_attrs(disabled);
    view! {
        <label
            data-rs-switch=""
            data-rs-component="Switch"
            data-rs-behavior="toggle"
            data-rs-state=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                data-rs-switch-input=""
                data-rs-disabled=d.data_rs_disabled
                name=if name.is_empty() { None } else { Some(name) }
                value=value
                aria-hidden="true"
                tabindex="-1"
            />
            {children()}
        </label>
    }
}

#[component]
pub fn SwitchThumbPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <span data-rs-switch-thumb="" class=class /> }
}
