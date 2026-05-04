//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, VisibilityState};

#[component]
pub fn ComboboxPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("cbx");
    view! {
        <div
            data-rs-combobox=""
            data-rs-uid=uid
            data-rs-interaction="selection"
            data-rs-visibility=state.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-name=name
            role="combobox"
            aria-expanded=state.aria_expanded()
            aria-haspopup="listbox"
            aria-disabled=disabled.aria_disabled()
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ComboboxInputPrimitive(
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <input
            data-rs-combobox-input=""
            type="text"
            placeholder=placeholder
            aria-autocomplete="list"
            autocomplete="off"
            aria-disabled=disabled.aria_disabled()
            class=class
        />
    }
}

#[component]
pub fn ComboboxListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-list=""
            role="listbox"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ComboboxItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-value=value
            role="option"
            tabindex="-1"
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-disabled=disabled.aria_disabled()
            class=class
        >
            {children()}
        </div>
    }
}
