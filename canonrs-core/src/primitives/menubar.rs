//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menubar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

#[component]
pub fn MenubarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar=""
            data-rs-uid=crate::infra::uid::generate("mb")
            data-rs-interaction="nav"
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar-menu="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarTriggerPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-menubar-trigger=""
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded="false"
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar-content=""
            role="menu"
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-menubar-item=""
            role="menuitem"
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
