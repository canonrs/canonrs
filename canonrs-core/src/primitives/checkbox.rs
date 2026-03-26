//! @canon-level: strict
//! @canon-owner: primitives-team
//! Checkbox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::state_engine::{activity_attrs, disabled_attrs};

#[component]
pub fn CheckboxPrimitive(
    #[prop(default = ActivityState::Inactive)] checked: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let a = activity_attrs(checked);
    let d = disabled_attrs(disabled);
    let is_checked = checked == ActivityState::Active;
    view! {
        <input
            type="checkbox"
            data-rs-checkbox-input=""
            data-rs-component="Checkbox"
            data-rs-behavior="form"
            data-rs-state=a.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            checked=is_checked
            disabled=d.disabled
            aria-checked=if is_checked { "true" } else { "false" }
            aria-disabled=d.aria_disabled
            name={if name.is_empty() { None } else { Some(name) }}
            class=class
        />
    }
}

#[component]
pub fn CheckboxIndicatorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-checkbox-indicator="" aria-hidden="true" class=class>
            {children()}
        </span>
    }
}
