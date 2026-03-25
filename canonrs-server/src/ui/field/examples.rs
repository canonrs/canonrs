use leptos::prelude::*;
use super::field_ui::*;
use crate::ui::input::Input;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Field>
            <FieldLabel>"Email"</FieldLabel>
            <Input
                name="email"
                placeholder="you@example.com"
            />
        </Field>
    }
}
