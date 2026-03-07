use leptos::prelude::*;
use super::field_ui::*;
use crate::ui::input::Input;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Field>
            <FieldLabel html_for="field-email".to_string()>"Email"</FieldLabel>
            <Input
                id="field-email".to_string()
                name="email".to_string()
                placeholder="you@example.com".to_string()
            />
        </Field>
    }
}
