//! @canon-level: strict
//! @canon-owner: primitives-team
//! Checkbox Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Default, Debug)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unchecked     => "unchecked",
            Self::Checked       => "checked",
            Self::Indeterminate => "indeterminate",
        }
    }
    pub fn aria_checked(&self) -> &'static str {
        match self {
            Self::Unchecked     => "false",
            Self::Checked       => "true",
            Self::Indeterminate => "mixed",
        }
    }
    pub fn is_checked(&self) -> bool {
        matches!(self, Self::Checked)
    }
}

#[component]
pub fn CheckboxPrimitive(
    children: Children,
    #[prop(default = CheckboxState::Unchecked)] checked: CheckboxState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let state = if disabled == DisabledState::Disabled {
        format!("{} disabled", checked.as_str())
    } else {
        checked.as_str().to_string()
    };
    view! {
        <label
            data-rs-checkbox=""
            data-rs-uid=crate::infra::uid::generate("cb")
            data-rs-interaction="init"
            data-rs-state=state
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                data-rs-checkbox-input=""
                checked=checked.is_checked()
                disabled=d.disabled
                aria-checked=checked.aria_checked()
                name={if name.is_empty() { None } else { Some(name) }}
            />
            {children()}
        </label>
    }
}

#[component]
pub fn CheckboxIndicatorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-checkbox-indicator="" aria-hidden="true" class=class>
            {children()}
        </span>
    }
}
