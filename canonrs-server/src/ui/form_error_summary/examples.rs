use leptos::prelude::*;
use super::{FormErrorSummary, FormError};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <FormErrorSummary errors=vec![
            FormError {
                field_label: "Email".to_string(),
                message: "Please enter a valid email address".to_string(),
            },
            FormError {
                field_label: "Password".to_string(),
                message: "Password must be at least 8 characters".to_string(),
            },
        ] />
    }
}

#[component]
pub fn custom_title_example() -> impl IntoView {
    view! {
        <FormErrorSummary
            errors=vec![
                FormError {
                    field_label: "Username".to_string(),
                    message: "Username is already taken".to_string(),
                },
            ]
            title="Form validation failed:".to_string()
        />
    }
}

#[component]
pub fn empty_state_example() -> impl IntoView {
    view! {
        <div>
            <p>"No errors - FormErrorSummary will not render"</p>
            <FormErrorSummary errors=vec![] />
        </div>
    }
}
