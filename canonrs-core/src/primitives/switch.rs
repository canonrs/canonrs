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
    let d   = disabled_attrs(disabled);
    let state = format!("{} {}", sel.data_rs_state.unwrap_or(""), if disabled == DisabledState::Disabled { "disabled" } else { "" }).trim().to_string();
    let is_checked = checked == SelectionState::Selected;
    let aria_checked = if is_checked { "true" } else { "false" };
    view! {
        <label
            data-rs-switch=""
            data-rs-uid=crate::infra::uid::generate("sw")
            data-rs-interaction="init"
            data-rs-state=state
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                role="switch"
                data-rs-switch-input=""
                name=if name.is_empty() { None } else { Some(name) }
                value=value
                checked=is_checked
                aria-checked=aria_checked
                tabindex="0"
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
