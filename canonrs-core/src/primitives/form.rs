//! @canon-level: strict
//! @canon-owner: primitives-team
//! Form Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::{disabled_attrs, validation_attrs};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FormMethod {
    #[default]
    Post,
    Get,
}
impl FormMethod {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Post => "post", Self::Get => "get" }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FormEnctype {
    #[default]
    UrlEncoded,
    Multipart,
    Text,
}
impl FormEnctype {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UrlEncoded => "application/x-www-form-urlencoded",
            Self::Multipart  => "multipart/form-data",
            Self::Text       => "text/plain",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FormValidationState {
    #[default]
    Idle,
    Submitting,
    Success,
    Error,
}
impl FormValidationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle       => "idle",
            Self::Submitting => "submitting",
            Self::Success    => "success",
            Self::Error      => "error",
        }
    }
}

#[component]
pub fn FormPrimitive(
    children: Children,
    #[prop(into, default = String::new())] action: String,
    #[prop(default = FormMethod::Post)] method: FormMethod,
    #[prop(default = FormEnctype::UrlEncoded)] enctype: FormEnctype,
    #[prop(default = FormValidationState::Idle)] validation: FormValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = true)] novalidate: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let v = validation_attrs(validation);
    let d = disabled_attrs(disabled);
    view! {
        <form
            data-rs-form=""
            data-rs-component="Form"
            data-rs-behavior="form"
            data-rs-state=v.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            action={if action.is_empty() { None } else { Some(action) }}
            method=method.as_str()
            enctype=enctype.as_str()
            novalidate=novalidate
            aria-busy=v.aria_busy
            aria-invalid=v.aria_invalid
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </form>
    }
}

#[component]
pub fn FormSectionPrimitive(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <section
            data-rs-form-section=""
            role="group"
            aria-label=aria_label
            class=class
        >
            {children()}
        </section>
    }
}

#[component]
pub fn FormActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-actions=""
            role="toolbar"
            class=class
        >
            {children()}
        </div>
    }
}

// ── Form Field System ─────────────────────────────────────────────────────

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
pub fn FormFieldPrimitive(
    children: Children,
    #[prop(default = FieldValidationState::Idle)] validation: FieldValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-form-field=""
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
pub fn FormLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(default = false)] required: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-form-label=""
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
pub fn FormErrorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-error=""
            role="alert"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FormHintPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-form-hint="" class=class>
            {children()}
        </div>
    }
}
