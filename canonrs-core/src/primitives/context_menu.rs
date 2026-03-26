//! @canon-level: strict
//! @canon-owner: primitives-team
//! ContextMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState, ActivityState};
use crate::state_engine::{visibility_attrs, disabled_attrs, activity_attrs};

#[component]
pub fn ContextMenuPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-context-menu=""
            data-rs-component="ContextMenu"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-trigger=""
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-context-menu-content=""
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
pub fn ContextMenuItemPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = ActivityState::Inactive)] highlighted: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let a = activity_attrs(highlighted);
    view! {
        <button
            type="button"
            data-rs-context-menu-item=""
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
pub fn ContextMenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-separator=""
            role="separator"
            class=class
        />
    }
}

#[component]
pub fn ContextMenuGroupPrimitive(
    children: Children,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-group=""
            role="group"
            aria-label=label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-label=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}
