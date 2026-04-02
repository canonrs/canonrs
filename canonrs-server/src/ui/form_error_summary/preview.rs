use leptos::prelude::*;
use super::form_error_summary_ui::{FormErrorSummary, FormError};

#[component]
pub fn FormErrorSummaryShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <FormErrorSummary
                    errors=vec![
                        FormError { field_label: "Email".to_string(), message: "Required field.".to_string() },
                        FormError { field_label: "Password".to_string(), message: "Must be at least 8 characters.".to_string() },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "All form errors announced together with structured summary."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Single error"</span>
                <div data-rs-showcase-preview-row="">
                    <FormErrorSummary
                        title="Please fix this error:"
                        errors=vec![
                            FormError { field_label: "Name".to_string(), message: "Cannot be empty.".to_string() },
                        ]
                    />
                </div>
            </div>
        </div>
    }
}
