use leptos::prelude::*;
use super::form_error_summary_ui::{FormErrorSummary, FormError};

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormErrorIslandItem {
    pub field_label: String,
    pub message:     String,
}

#[island]
pub fn FormErrorSummaryIsland(
    #[prop(optional)] errors: Option<Vec<FormErrorIslandItem>>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let title  = title.unwrap_or_else(|| "Please fix the following errors:".to_string());
    let class  = class.unwrap_or_default();
    let errors = errors.unwrap_or_default()
        .into_iter()
        .map(|e| FormError { field_label: e.field_label, message: e.message })
        .collect::<Vec<_>>();

    view! {
        <FormErrorSummary
            errors=errors
            title=title
            class=class
        />
    }
}
