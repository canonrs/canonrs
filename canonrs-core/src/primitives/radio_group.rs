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
    view! {
        <label
            data-rs-radio=""
            data-rs-component="RadioGroupItem"
            data-rs-state=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            class=class
        >
            <input
                type="radio"
                data-rs-radio-input=""
                name=name
                value=value
                checked=sel.aria_selected
                aria-checked=sel.aria_selected
                aria-disabled=d.aria_disabled
            />
            <span data-rs-radio-indicator="" />
            {children()}
        </label>
    }
}
