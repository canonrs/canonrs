use leptos::callback::Callback;
use leptos::prelude::*;
use canonrs_ui::ui::form_error_summary::FormError;
use canonrs_ui::primitives::form_error_summary::FormErrorSummaryPrimitive;

#[component]
pub fn FormErrorSummaryInteractive(
    errors: Signal<Vec<FormError>>,
    #[prop(default = "Please fix the following errors:".to_string())] title: String,
    #[prop(optional)] on_field_click: Option<Callback<String>>,
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
                        let field_id = error.field_id.clone();
                        view! {
                            <li attr:data-form-error-summary-item="">
                                <button
                                    attr:data-form-error-summary-item-link=""
                                    type="button"
                                    on:click=move |_| {
                                        if let Some(ref handler) = on_field_click {
                                            handler.run(field_id.clone());
                                        }
                                    }
                                >
                                    {error.field_label.clone()}{": "}{error.message.clone()}
                                </button>
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </FormErrorSummaryPrimitive>
        })}
    }
}
