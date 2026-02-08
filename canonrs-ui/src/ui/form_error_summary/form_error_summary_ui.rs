use leptos::prelude::*;
use crate::primitives::form_error_summary::FormErrorSummaryPrimitive;

#[derive(Clone, Debug, PartialEq)]
pub struct FormError {
    pub field_id: String,
    pub field_label: String,
    pub message: String,
}

#[component]
pub fn FormErrorSummary(
    errors: Signal<Vec<FormError>>,
    #[prop(default = "Please fix the following errors:".to_string())] title: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let has_errors = move || !errors.get().is_empty();
    let title = StoredValue::new(title);
    let class = StoredValue::new(class);
    let id = StoredValue::new(id);

    view! {
        {move || has_errors().then(|| view! {
            <FormErrorSummaryPrimitive class={class.get_value()} id={id.get_value()}>
                <h3 attr:data-form-error-summary-title="">{title.get_value()}</h3>

                <ul attr:data-form-error-summary-list="">
                    {move || errors.get().into_iter().map(|error| {
                        view! {
                            <li attr:data-form-error-summary-item="">
                                <span attr:data-form-error-summary-item-link="">
                                    {error.field_label.clone()}{": "}{error.message.clone()}
                                </span>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </FormErrorSummaryPrimitive>
        })}
    }
}
