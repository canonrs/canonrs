//! @canon-level: strict
//! @canon-owner: primitives-team
//! RadioGroup Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};
use crate::infra::state_engine::{selection_attrs, disabled_attrs};










#[component]
pub fn RadioGroupPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-radio-group=""
            data-rs-uid=crate::infra::uid::generate("rg")
            data-rs-interaction="selection"
            data-rs-component="RadioGroup"
            data-rs-disabled=d.data_rs_disabled
            role="radiogroup"
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn RadioGroupItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sel = selection_attrs(selected);
    let d = disabled_attrs(disabled);

    let state_str: Option<String> = match (sel.data_rs_state, disabled == DisabledState::Disabled) {
        (Some(s), true)  => Some(format!("{} disabled", s)),
        (Some(s), false) => Some(s.to_string()),
        (None,    true)  => Some("disabled".to_string()),
        (None,    false) => None,
    };

    view! {
        <label
            data-rs-radio=""
            data-rs-component="RadioGroupItem"
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
