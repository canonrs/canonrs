//! @canon-level: strict
//! @canon-owner: primitives-team
//! Field Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FieldValidationState {
    #[default]
    Idle,
    Valid,
    Invalid,
    Warning,
}

impl FieldValidationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle    => "idle",
            Self::Valid   => "valid",
            Self::Invalid => "invalid",
            Self::Warning => "warning",
        }
    }

    pub fn aria_invalid(&self) -> Option<&'static str> {
        match self { Self::Invalid => Some("true"), _ => None }
    }
}

#[component]
pub fn FieldPrimitive(
    children: Children,
    #[prop(default = FieldValidationState::Idle)] validation: FieldValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-field=""
            data-rs-uid=crate::infra::uid::generate("fi")
            data-rs-interaction="init"
            data-rs-state=validation.as_str()
            data-rs-disabled=d.data_rs_disabled
            aria-invalid=validation.aria_invalid()
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(default = false)] required: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-field-label=""
            for={if html_for.is_empty() { None } else { Some(html_for) }}
            data-rs-required={if required { Some("") } else { None }}
            aria-required={if required { Some("true") } else { None }}
            class=class
        >
            {children()}
        </label>
    }
}

#[component]
pub fn FieldDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-field-description="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn FieldErrorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-field-error=""
            role="alert"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldGroupPrimitive(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-field-group=""
            role="group"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldSetPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <fieldset data-rs-fieldset="" class=class>
            {children()}
        </fieldset>
    }
}

#[component]
pub fn FieldLegendPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <legend data-rs-field-legend="" class=class>
            {children()}
        </legend>
    }
}
