//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, VisibilityState};
use crate::infra::state_engine::{disabled_attrs, selection_attrs, visibility_attrs};

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
    let d = disabled_attrs(disabled);
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-combobox=""
            data-rs-uid=uid
            data-rs-interaction="selection"
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-name=name
            role="combobox"
            aria-expanded=s.aria_expanded
            aria-haspopup="listbox"
            aria-disabled=d.aria_disabled
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
    let d = disabled_attrs(disabled);
    view! {
        <input
            data-rs-combobox-input=""
            type="text"
            placeholder=placeholder
            aria-autocomplete="list"
            autocomplete="off"
            aria-disabled=d.aria_disabled
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
    let sel = selection_attrs(selected);
    let d   = disabled_attrs(disabled);
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-state=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-value=value
            role="option"
            tabindex="-1"
            aria-selected=sel.aria_selected
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}
