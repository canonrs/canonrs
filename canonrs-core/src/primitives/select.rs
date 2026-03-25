//! @canon-level: strict
//! @canon-owner: primitives-team
//! Select Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState, StateKind};
use crate::state_engine::resolve_state;

#[derive(Clone, PartialEq, Default, Debug)]
pub enum SelectSize {
    #[default]
    Md,
    Sm,
    Lg,
}
impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Md => "md",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }
}

#[component]
pub fn SelectPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = SelectSize::Md)] size: SelectSize,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    view! {
        <div
            data-rs-select=""
            data-rs-component="Select"
            data-rs-behavior="selection"
            data-rs-state=state.as_str()
            data-rs-size=size.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SelectTriggerPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-select-trigger=""
            aria-haspopup="listbox"
            aria-expanded={if state == VisibilityState::Open { "true" } else { "false" }}
            data-rs-state={if disabled.as_bool() { resolve_state(StateKind::Toggle(crate::meta::ToggleState::Off)) } else { state.as_str() }}
            aria-disabled=disabled.aria()
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
        <span data-rs-select-value="" data-rs-placeholder=placeholder class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn SelectContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-select-content=""
            data-rs-state=state.as_str()
            role="listbox"
            hidden={state == VisibilityState::Closed}
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
    view! {
        <div
            data-rs-select-item=""
            data-rs-state={if disabled.as_bool() { disabled.as_str() } else { resolve_state(StateKind::Selection(selected)) }}
            data-rs-value=value
            role="option"
            tabindex=-1
            aria-selected={if selected == SelectionState::Selected { Some("true") } else { None }}
            aria-disabled=disabled.aria()
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
        <div data-rs-select-separator="" role="separator" class=class />
    }
}
