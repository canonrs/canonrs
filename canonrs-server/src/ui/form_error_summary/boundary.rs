//! FormErrorSummary Island — Canon Rule passthrough
use leptos::prelude::*;
use super::form_error_summary_ui::FormError;

#[component]
pub fn FormErrorSummary(
    #[prop(optional)] errors: Option<Vec<FormError>>,
    #[prop(into, default = String::from("Please fix the following errors:"))] title: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <super::form_error_summary_ui::FormErrorSummary
            errors=errors.unwrap_or_default()
            title=title
            class=class
        />
}
}
