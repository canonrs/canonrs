//! @canon-level: strict
//! @canon-owner: primitives-team
//! Form Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;

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
    pub fn aria_busy(&self) -> Option<&'static str> {
        if *self == Self::Submitting { Some("true") } else { None }
    }
    pub fn aria_invalid(&self) -> Option<&'static str> {
        if *self == Self::Error { Some("true") } else { None }
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
    let uid_fo = crate::infra::uid::generate("fo");
    view! {
        <form
            data-rs-form=""
            data-rs-uid=uid_fo
            data-rs-interaction="init"
            data-rs-validation=validation.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            action={if action.is_empty() { None } else { Some(action) }}
            method=method.as_str()
            enctype=enctype.as_str()
            novalidate=novalidate
            aria-busy=validation.aria_busy()
            aria-invalid=validation.aria_invalid()
            aria-disabled=disabled.aria_disabled()
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
            aria-label=aria_label.unwrap_or_default()
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
    children: ChildrenFn,
    #[prop(default = FieldValidationState::Idle)] validation: FieldValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-field=""
            data-rs-validation=validation.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-invalid=validation.aria_invalid()
            aria-disabled=disabled.aria_disabled()
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
            for=html_for
            data-rs-required={if required { "true" } else { "false" }}
            aria-required={if required { "true" } else { "false" }}
            class=class
        >
            <span data-rs-label-text="">{children()}</span>
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
            aria-live="assertive"
            class=class
        >
            <span data-rs-error-text="">{children()}</span>
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
            <span data-rs-form-hint-text="">{children()}</span>
        </div>
    }
}
