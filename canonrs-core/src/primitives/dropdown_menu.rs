//! @canon-level: strict
//! @canon-owner: primitives-team
//! DropdownMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{DisabledState, ToggleState, VisibilityState};
use crate::infra::state_engine::{disabled_attrs, toggle_attrs, visibility_attrs};

#[component]
pub fn DropdownMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-component="DropdownMenu"
            data-rs-state=s.data_rs_state
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-trigger=""
            aria-haspopup="menu"
            aria-expanded="false"
            aria-disabled=aria_disabled
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-content=""
            role="menu"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-group=""
            role="group"
            aria-label=label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let state_str = if disabled == DisabledState::Disabled {
        "inactive disabled"
    } else {
        "inactive"
    };
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-item=""
            role="menuitem"
            data-rs-state=state_str
            aria-disabled=d.aria_disabled
            tabindex=if d.disabled { "-1" } else { "0" }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-separator=""
            role="separator"
            class=class
        />
    }
}

#[component]
pub fn DropdownMenuCheckboxItemPrimitive(
    children: Children,
    #[prop(default = ToggleState::Off)] checked: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = toggle_attrs(checked);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-checkbox-item=""
            data-rs-state=t.data_rs_state
            role="menuitemcheckbox"
            aria-checked=t.aria_pressed
            aria-disabled=d.aria_disabled
            tabindex=if d.disabled { "-1" } else { "0" }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-label=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}
