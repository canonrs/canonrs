use leptos::prelude::*;
use canonrs_core::primitives::FormErrorSummaryPrimitive;

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
    let errors = errors.unwrap_or_default();

    if errors.is_empty() {
        return view! { <div></div> }.into_any();
    }

    view! {
        <FormErrorSummaryPrimitive class=class>
            <h3 data-rs-form-error-summary-title="">{title}</h3>
            <ul data-rs-form-error-summary-list="">
                {errors.into_iter().map(|error| view! {
                    <li data-rs-form-error-summary-item="">
                        <span data-rs-form-error-summary-item-link="">
                            {error.field_label}{": "}{error.message}
                        </span>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </FormErrorSummaryPrimitive>
    }.into_any()
}
