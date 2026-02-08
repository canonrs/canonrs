use leptos::prelude::*;
use super::{FormErrorSummary, FormError};

#[component]
pub fn basic_example() -> impl IntoView {
    let errors = RwSignal::new(vec![
        FormError {
            field_id: "email".to_string(),
            field_label: "Email".to_string(),
            message: "Please enter a valid email address".to_string(),
        },
        FormError {
            field_id: "password".to_string(),
            field_label: "Password".to_string(),
            message: "Password must be at least 8 characters".to_string(),
        },
    ]);

    view! {
        <FormErrorSummary errors=errors.into() />
    }
}

#[component]
pub fn custom_title_example() -> impl IntoView {
    let errors = RwSignal::new(vec![
        FormError {
            field_id: "username".to_string(),
            field_label: "Username".to_string(),
            message: "Username is already taken".to_string(),
        },
    ]);

    view! {
        <FormErrorSummary
            errors=errors.into()
            title="Form validation failed:".to_string()
        />
    }
}

#[component]
pub fn empty_state_example() -> impl IntoView {
    let errors = RwSignal::new(vec![]);

    view! {
        <div>
            <p>"No errors - FormErrorSummary will not render"</p>
            <FormErrorSummary errors=errors.into() />
        </div>
    }
}
