//! @canon-level: strict
//! @canon-owner: primitives-team
//! ColorPicker Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, VisibilityState};

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
    view! {
        <div
            data-rs-color-picker=""
            data-rs-uid=uid
            data-rs-interaction="selection"
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-name=name
            data-rs-value=value
            aria-disabled=disabled.aria_disabled()
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
    view! {
        <button
            type="button"
            data-rs-color-picker-trigger=""
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-color=color
            aria-haspopup="dialog"
            aria-expanded=state.aria_expanded()
            aria-disabled=disabled.aria_disabled()
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
    view! {
        <input
            type="color"
            data-rs-color-picker-input=""
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            disabled=disabled.disabled()
            value=value
            name=name
            aria-label="Color picker"
            aria-disabled=disabled.aria_disabled()
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
    view! {
        <button
            type="button"
            data-rs-color-swatch=""
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-color=color
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-label=aria_label
            class=class
        />
    }
}
