//! @canon-level: strict
//! @canon-owner: primitives-team
//! Combobox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};
use crate::infra::state_engine::selection_attrs;
use std::sync::atomic::{AtomicU64, Ordering};

static COMBOBOX_CTR: AtomicU64 = AtomicU64::new(0);

fn combobox_uid() -> String {
    // UUID v4 simplificado: timestamp + contador atomico
    let ctr = COMBOBOX_CTR.fetch_add(1, Ordering::SeqCst);
    format!("cbx-{:016x}-{:08x}", 
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(ctr),
        ctr)
}

#[component]
pub fn ComboboxPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    let state_str = if disabled == DisabledState::Disabled { "closed disabled" } else { "closed" };
    view! {
        <div
            data-rs-combobox=""
            data-rs-uid=combobox_uid()
            data-rs-interaction="selection"
            data-rs-component="Combobox"
            data-rs-role="root"
            data-rs-state=state_str
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
            data-rs-component="ComboboxInput"
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
            data-rs-component="ComboboxList"
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
            data-rs-component="ComboboxItem"
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
