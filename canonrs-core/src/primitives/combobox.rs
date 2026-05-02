//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};
use crate::infra::state_engine::selection_attrs;







#[component]
pub fn ComboboxPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    let uid_cbx = crate::infra::uid::generate("cbx");
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    let state_str = if disabled == DisabledState::Disabled { "closed disabled" } else { "closed" };
    view! {
        <div
            data-rs-combobox=""
            data-rs-uid=uid_cbx
            data-rs-interaction="selection"
            data-rs-role="root"
            data-rs-state=state_str
            data-rs-name=name
            role="combobox"
            aria-expanded="false"
            aria-haspopup="listbox"
            aria-disabled=aria_disabled
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
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <input
            data-rs-combobox-input=""
            type="text"
            placeholder=placeholder
            aria-autocomplete="list"
            autocomplete="off"
            aria-disabled=aria_disabled
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
            data-rs-role="list"
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
    let base = sel.data_rs_state.unwrap_or("unselected");
    let state_str = if disabled == DisabledState::Disabled {
        format!("{} disabled", base)
    } else {
        base.to_string()
    };
    let aria_selected = sel.aria_selected.unwrap_or("false");
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <div
            data-rs-combobox-item=""
            data-rs-state=state_str
            data-rs-value=value
            role="option"
            tabindex="-1"
            aria-selected=aria_selected
            aria-disabled=aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}
