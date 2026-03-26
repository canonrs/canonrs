//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menubar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState, ActivityState};
use crate::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs, activity_attrs};

#[component]
pub fn MenubarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar=""
            data-rs-component="Menubar"
            data-rs-behavior="navigation"
            role="menubar"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarMenuPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-menubar-menu=""
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarTriggerPrimitive(
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
            data-rs-menubar-trigger=""
            role="menuitem"
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
pub fn MenubarContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-menubar-content=""
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
pub fn MenubarItemPrimitive(
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
            data-rs-menubar-item=""
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
pub fn MenubarSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar-separator="" role="separator" class=class />
    }
}

#[component]
pub fn MenubarLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar-label=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}
