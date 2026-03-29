//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState};
use crate::infra::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs, selection_attrs};

#[component]
pub fn ComboboxPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-combobox=""
            data-rs-component="Combobox"
            data-rs-role="root"
            data-rs-behavior="combobox"
            data-rs-state=s.data_rs_state
            role="combobox"
            aria-controls="combobox-list"
            aria-expanded=s.aria_expanded
            aria-haspopup="listbox"
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            <input
                data-rs-combobox-input=""
                data-rs-component="ComboboxInput"
                type="text"
                aria-autocomplete="list"
                autocomplete="off"
            />
            {children()}
        </div>
    }
}

#[component]
pub fn ComboboxTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = trigger_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-combobox-trigger=""
            data-rs-component="ComboboxTrigger"
            aria-haspopup="listbox"
            aria-expanded=t.aria_expanded
            data-rs-state=t.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ComboboxListPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-combobox-list=""
            data-rs-role="list"
            id="combobox-list"
            data-rs-component="ComboboxList"
            data-rs-state=s.data_rs_state
            role="listbox"
            hidden=s.hidden
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
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-component="ComboboxItem"
            data-rs-state=sel.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            data-rs-value={value}
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
