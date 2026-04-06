use leptos::prelude::*;
use super::field_ui::Field;
use super::variants::{FieldOrientation, FieldValidation};

#[island]
pub fn FieldIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let class    = class.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    view! {
        <Field
            orientation=FieldOrientation::Vertical
            _validation=FieldValidation::None
            disabled=disabled
            class=class
        >
            {children()}
        </Field>
    }
}
