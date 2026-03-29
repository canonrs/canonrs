//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState};
use crate::infra::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs, selection_attrs};

#[component]
pub fn SelectPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-select=""
            data-rs-component="Select"
            data-rs-role="root"
            data-rs-behavior="select"
            data-rs-state=s.data_rs_state
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SelectTriggerPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = trigger_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-select-trigger=""
            aria-haspopup="listbox"
            aria-expanded=t.aria_expanded
            data-rs-state=t.data_rs_state
            data-rs-component="SelectTrigger"
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SelectValuePrimitive(
    children: Children,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-select-value="" data-rs-component="SelectValue" data-rs-placeholder=placeholder class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn SelectContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-select-content=""
            data-rs-component="SelectContent"
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
pub fn SelectItemPrimitive(
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
            data-rs-select-item=""
            data-rs-component="SelectItem"
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

#[component]
pub fn SelectSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-select-separator="" data-rs-component="SelectSeparator" role="separator" class=class />
    }
}
