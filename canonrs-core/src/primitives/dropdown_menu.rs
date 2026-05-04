//! @canon-level: strict
//! @canon-owner: primitives-team
//! DropdownMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{DisabledState, ToggleState, VisibilityState};



#[component]
pub fn DropdownMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let uid_dm = crate::infra::uid::generate("dm");
    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-uid=uid_dm
            data-rs-interaction="overlay"
            data-rs-visibility=state.as_str()
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
    let is_disabled = disabled == DisabledState::Disabled;
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-item=""
            role="menuitem"
            data-rs-activity="inactive"
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            aria-disabled=if is_disabled { Some("true") } else { None }
            tabindex=if is_disabled { "-1" } else { "0" }
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
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-checkbox-item=""
            data-rs-toggle=checked.as_str()
            role="menuitemcheckbox"
            aria-checked=checked.aria_pressed()
            aria-disabled=disabled.aria_disabled()
            tabindex=if disabled.disabled() { "-1" } else { "0" }
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
