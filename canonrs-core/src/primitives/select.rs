//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState};

#[component]
pub fn SelectPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    let uid_sel = crate::infra::uid::generate("sel");
    let is_disabled = disabled == DisabledState::Disabled;
    let is_open = state == VisibilityState::Open;

    view! {
        <div
            data-rs-select=""
            data-rs-uid=uid_sel
            data-rs-interaction="selection"
            data-rs-role="root"
            data-rs-visibility=if is_open { "open" } else { "closed" }
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            data-rs-name=name
            aria-disabled=if is_disabled { "true" } else { "false" }
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
    let is_disabled = disabled == DisabledState::Disabled;
    view! {
        <button
            type="button"
            data-rs-select-trigger=""
            aria-haspopup="listbox"
            aria-expanded="false"
            aria-disabled=if is_disabled { "true" } else { "false" }
            class=class
        >
            {children()}
            <svg data-rs-select-chevron="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="m6 9 6 6 6-6"/></svg>
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
    let is_selected = selected == SelectionState::Selected;
    let is_disabled = disabled == DisabledState::Disabled;

    view! {
        <div
            data-rs-select-item=""
            data-rs-selection=if is_selected { Some("selected") } else { None }
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            data-rs-value=value
            role="option"
            tabindex="-1"
            aria-selected=if is_selected { "true" } else { "false" }
            aria-disabled=if is_disabled { "true" } else { "false" }
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
            role="separator"
            class=class
        />
    }
}
