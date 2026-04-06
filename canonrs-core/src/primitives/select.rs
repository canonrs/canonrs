//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState};
use crate::infra::state_engine::{visibility_attrs, selection_attrs};

#[component]
pub fn SelectPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let state_str = if disabled == DisabledState::Disabled {
        format!("{} disabled", s.data_rs_state)
    } else {
        s.data_rs_state.to_string()
    };
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <div
            data-rs-select=""
            data-rs-component="Select"
            data-rs-role="root"
            data-rs-state=state_str
            aria-disabled=aria_disabled
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <button
            type="button"
            data-rs-select-trigger=""
            data-rs-component="SelectTrigger"
            aria-haspopup="listbox"
            aria-expanded="false"
            aria-disabled=aria_disabled
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
        <span
            data-rs-select-value=""
            data-rs-component="SelectValue"
            data-rs-placeholder=placeholder
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn SelectContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-select-content=""
            data-rs-component="SelectContent"
            role="listbox"
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
            data-rs-select-item=""
            data-rs-component="SelectItem"
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

#[component]
pub fn SelectSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-select-separator=""
            data-rs-component="SelectSeparator"
            role="separator"
            class=class
        />
    }
}
