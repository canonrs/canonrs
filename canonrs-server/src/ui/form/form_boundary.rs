//! @canon-level: strict
//! Form Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
pub use canonrs_core::primitives::{FormValidationState, FormMethod, FormEnctype};
pub use super::form_ui::FieldValidationState;
pub use canonrs_core::meta::DisabledState;
use super::form_ui::{
    Form as FormUi, FormSection as FormSectionUi, FormActions as FormActionsUi,
    FormField as FormFieldUi, FormLabel as FormLabelUi,
    FormHint as FormHintUi, FormError as FormErrorUi,
};

#[component]
pub fn Form(
    children: Children,
    #[prop(into, default = String::new())] action: String,
    #[prop(default = FormMethod::Post)] method: FormMethod,
    #[prop(default = FormEnctype::UrlEncoded)] enctype: FormEnctype,
    #[prop(default = FormValidationState::Idle)] validation: FormValidationState,
) -> impl IntoView {
    view! { <FormUi action=action method=method enctype=enctype validation=validation>{children()}</FormUi> }
}

#[component]
pub fn FormSection(
    children: Children,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let aria = aria_label.unwrap_or_default();
    view! { <FormSectionUi aria_label=aria class=class>{children()}</FormSectionUi> }
}

#[component]
pub fn FormActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <FormActionsUi class=class>{children()}</FormActionsUi> }
}

#[component]
pub fn FormField(
    children: ChildrenFn,
    #[prop(default = FieldValidationState::Idle)] validation: FieldValidationState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <FormFieldUi validation=validation disabled=disabled class=class>{children()}</FormFieldUi> }
}

#[component]
pub fn FormLabel(
    children: Children,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(default = false)] required: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <FormLabelUi html_for=html_for required=required class=class>{children()}</FormLabelUi> }
}

#[component]
pub fn FormHint(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <FormHintUi class=class>{children()}</FormHintUi> }
}

#[component]
pub fn FormError(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <FormErrorUi class=class>{children()}</FormErrorUi> }
}
