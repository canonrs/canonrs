//! @canon-level: strict
//! @canon-owner: primitives-team
//! DropdownMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState, ToggleState, ActivityState};
use crate::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs, toggle_attrs, activity_attrs};

#[component]
pub fn DropdownMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-component="DropdownMenu"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    let t = trigger_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-trigger=""
            aria-haspopup="menu"
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
pub fn DropdownMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-dropdown-menu-content=""
            data-rs-state=s.data_rs_state
            role="menu"
            hidden=s.hidden
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
    #[prop(default = ActivityState::Inactive)] highlighted: ActivityState,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let a = activity_attrs(highlighted);
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-item=""
            role="menuitem"
            data-rs-state=a.data_rs_state
            data-rs-disabled=d.data_rs_disabled
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
    #[prop(default = ActivityState::Inactive)] highlighted: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = toggle_attrs(checked);
    let d = disabled_attrs(disabled);
    let a = activity_attrs(highlighted);
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-checkbox-item=""
            data-rs-state=t.data_rs_state
            data-rs-highlighted=a.data_rs_state
            role="menuitemcheckbox"
            aria-checked=t.aria_pressed
            data-rs-disabled=d.data_rs_disabled
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
