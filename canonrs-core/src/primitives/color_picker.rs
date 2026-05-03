//! @canon-level: strict
//! @canon-owner: primitives-team
//! ColorPicker Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, VisibilityState};
use crate::infra::state_engine::{disabled_attrs, selection_attrs, visibility_attrs};

#[component]
pub fn ColorPickerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("cp");
    let s = visibility_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-color-picker=""
            data-rs-uid=uid
            data-rs-interaction="selection"
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-name=name
            data-rs-value=value
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ColorPickerTriggerPrimitive(
    children: Children,
    #[prop(into, default = "#000000".to_string())] color: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-color-picker-trigger=""
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-color=color
            aria-haspopup="dialog"
            aria-expanded=s.aria_expanded
            aria-disabled=d.aria_disabled
            aria-label="Open color picker"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ColorPickerInputPrimitive(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <input
            type="color"
            data-rs-color-picker-input=""
            data-rs-disabled=d.data_rs_disabled
            disabled=d.disabled
            value=value
            name=name
            aria-label="Color picker"
            aria-disabled=d.aria_disabled
            class=class
        />
    }
}

#[component]
pub fn ColorPickerSwatchPrimitive(
    #[prop(into, default = "#000000".to_string())] color: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let sel = selection_attrs(selected);
    view! {
        <button
            type="button"
            data-rs-color-swatch=""
            data-rs-state=sel.data_rs_state
            data-rs-color=color
            aria-selected=sel.aria_selected
            aria-label=aria_label
            class=class
        />
    }
}
