//! @canon-id: form-error-summary
//! @canon-label: Form Error Summary
//! @canon-family: utility
//! @canon-category: Form
//! @canon-intent: Summarize form validation errors
//! @canon-description: Form validation error summary
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: form-error-summary, error, validation, form, summary

use leptos::prelude::*;
use canonrs_core::primitives::FormErrorSummaryPrimitive;

#[derive(Clone, Debug, PartialEq)]
pub struct FormError {
    pub field_label: String,
    pub message: String,
}

#[component]
pub fn FormErrorSummary(
    #[prop(default = vec![])] errors: Vec<FormError>,
    #[prop(into, default = "Please fix the following errors:".to_string())] title: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormErrorSummaryPrimitive class=class>
            <h3 data-rs-form-error-summary-title="">{title}</h3>
            <ul data-rs-form-error-summary-list="">
                {errors.into_iter().map(|error: FormError| {
                    view! {
                        <li data-rs-form-error-summary-item="">
                            <span data-rs-form-error-summary-item-link="">
                                {error.field_label}{": "}{error.message}
                            </span>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </FormErrorSummaryPrimitive>
    }
}

#[component]
pub fn FormErrorSummaryPreview() -> impl IntoView {
    view! {
        <FormErrorSummary errors=vec![
            FormError { field_label: "Email".to_string(), message: "Required".to_string() },
        ] />
    }
}
