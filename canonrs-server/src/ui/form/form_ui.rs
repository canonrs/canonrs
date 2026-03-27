//! @canon-id: form
//! @canon-label: Form
//! @canon-family: interactive
//! @canon-category: Form
//! @canon-intent: Form container with submit handling
//! @canon-description: Form component
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: form, submit, validation, input, fields

use leptos::prelude::*;
use canonrs_core::primitives::{
    FormPrimitive, FormSectionPrimitive, FormActionsPrimitive,
    FormFieldPrimitive, FormLabelPrimitive, FormErrorPrimitive, FormHintPrimitive,
};
pub use canonrs_core::primitives::{
    FormMethod, FormEnctype, FormValidationState, FieldValidationState,
};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Form(
    children: Children,
    #[prop(into, default = String::new())] action: String,
    #[prop(default = FormMethod::Post)] method: FormMethod,
    #[prop(default = FormEnctype::UrlEncoded)] enctype: FormEnctype,
    #[prop(default = FormValidationState::Idle)] validation: FormValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = true)] novalidate: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormPrimitive
            action=action method=method enctype=enctype
            validation=validation disabled=disabled
            novalidate=novalidate class=class
        >
            {children()}
        </FormPrimitive>
    }
}

#[component]
pub fn FormSection(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormSectionPrimitive aria_label=aria_label class=class>
            {children()}
        </FormSectionPrimitive>
    }
}

#[component]
pub fn FormActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormActionsPrimitive class=class>
            {children()}
        </FormActionsPrimitive>
    }
}

#[component]
pub fn FormField(
    children: Children,
    #[prop(default = FieldValidationState::Idle)] validation: FieldValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormFieldPrimitive validation=validation disabled=disabled class=class>
            {children()}
        </FormFieldPrimitive>
    }
}

#[component]
pub fn FormLabel(
    children: Children,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(default = false)] required: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormLabelPrimitive html_for=html_for required=required class=class>
            {children()}
        </FormLabelPrimitive>
    }
}

#[component]
pub fn FormError(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormErrorPrimitive class=class>
            {children()}
        </FormErrorPrimitive>
    }
}

#[component]
pub fn FormHint(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormHintPrimitive class=class>
            {children()}
        </FormHintPrimitive>
    }
}

#[component]
pub fn FormPreview() -> impl IntoView {
    view! {
        <Form>
            <FormField>
                <FormLabel html_for="name" required=true>"Name"</FormLabel>
                <FormHint>"Your full name"</FormHint>
            </FormField>
            <FormActions>
                <button type="submit">"Submit"</button>
            </FormActions>
        </Form>
    }
}
