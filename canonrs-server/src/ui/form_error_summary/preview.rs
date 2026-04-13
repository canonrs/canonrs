use leptos::prelude::*;
use super::form_error_summary_boundary::{FormErrorSummary, FormError};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn FormErrorSummaryShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <FormErrorSummary
                errors=vec![
                    FormError { field_label: "Email".to_string(),    message: "Required field.".to_string() },
                    FormError { field_label: "Password".to_string(), message: "Must be at least 8 characters.".to_string() },
                ]
            />
            <p data-rs-showcase-preview-anchor="">
                "All form errors announced together with structured summary."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Single error"</span>
                <FormErrorSummary
                    title="Please fix this error:"
                    errors=vec![
                        FormError { field_label: "Name".to_string(), message: "Cannot be empty.".to_string() },
                    ]
                />
            </Stack>
        </Stack>
    }
}
