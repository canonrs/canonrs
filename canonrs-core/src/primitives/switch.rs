//! @canon-level: strict
//! @canon-owner: primitives-team
//! Switch Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};


#[component]
pub fn SwitchPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] checked: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_sw = crate::infra::uid::generate("sw");
    let is_checked = checked == SelectionState::Selected;
    let is_disabled = disabled == DisabledState::Disabled;
    let aria_checked = if is_checked { "true" } else { "false" };
    view! {
        <label
            data-rs-switch=""
            data-rs-uid=uid_sw
            data-rs-interaction="init"
            data-rs-selection=if checked == SelectionState::Selected { Some("selected") } else { None }
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-disabled=disabled.aria_disabled()
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
