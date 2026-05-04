//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState};

#[component]
pub fn MenuPrimitive(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_mn = crate::infra::uid::generate("mn");
    view! {
        <nav
            data-rs-menu=""
            data-rs-uid=uid_mn
            data-rs-interaction="init"
            aria-label=aria_label
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn MenuItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_mi = crate::infra::uid::generate("mi");
    view! {
        <button
            type="button"
            data-rs-menu-item=""

                        data-rs-uid=uid_mi
            role="menuitem"
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-disabled=disabled.aria_disabled()
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenuGroupPrimitive(
    children: Children,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menu-group=""
            role="group"
            aria-label=label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menu-label=""
            role="presentation"
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menu-separator="" role="separator" class=class />
    }
}
