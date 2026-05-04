//! @canon-level: strict
//! @canon-owner: primitives-team
//! RadioGroup Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioGroupPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_rg = crate::infra::uid::generate("rg");
    let is_disabled = disabled == DisabledState::Disabled;

    view! {
        <div
            data-rs-radio-group=""
            data-rs-uid=uid_rg
            data-rs-interaction="selection"
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            role="radiogroup"
            aria-disabled=if is_disabled { Some("true") } else { None }
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
    let is_selected = selected == SelectionState::Selected;
    let is_disabled = disabled == DisabledState::Disabled;

    view! {
        <label
            data-rs-radio=""
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
