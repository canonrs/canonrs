//! @canon-level: strict
//! @canon-owner: primitives-team
//! Checkbox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::infra::state_engine::{activity_attrs, disabled_attrs};

#[component]
pub fn CheckboxPrimitive(
    children: Children,
    #[prop(default = ActivityState::Inactive)] checked: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let a = activity_attrs(checked);
    let d = disabled_attrs(disabled);
    let is_checked = checked == ActivityState::Active;
    let state = if disabled == DisabledState::Disabled {
        format!("{} disabled", a.data_rs_state)
    } else {
        a.data_rs_state.to_string()
    };

    view! {
        <label
            data-rs-checkbox=""
            data-rs-uid=crate::infra::uid::generate("cb")
            data-rs-interaction="init"
            data-rs-state=state
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                data-rs-checkbox-input=""
                checked=is_checked
                disabled=d.disabled
                name={if name.is_empty() { None } else { Some(name) }}
            />
            {children()}
        </label>
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
