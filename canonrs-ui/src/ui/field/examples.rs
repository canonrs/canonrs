use leptos::prelude::*;
use super::field_ui::*;
use crate::ui::input::Input;

pub fn basic_example() -> impl IntoView {
    view! {
        <Field>
            <FieldLabel>"Email"</FieldLabel>
            <Input placeholder="you@example.com".to_string() />
        </Field>
    }
}
