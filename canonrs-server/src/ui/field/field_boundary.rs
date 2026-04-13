//! @canon-level: strict
//! Field Island — Canon Rule #340 (zero-logic boundary)

pub use super::field_ui::{FieldLabel, FieldDescription, FieldError, FieldSet, FieldGroup, FieldContent};
pub use super::variants::{FieldOrientation, FieldValidation};
use leptos::prelude::*;
use super::field_ui::Field as FieldUi;

#[component]
pub fn Field(
    children: Children,
    #[prop(default = FieldOrientation::Vertical)] orientation: FieldOrientation,
    #[prop(default = FieldValidation::None)] _validation: FieldValidation,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FieldUi orientation=orientation _validation=_validation disabled=disabled class=class>
            {children()}
        </FieldUi>
    }
}
