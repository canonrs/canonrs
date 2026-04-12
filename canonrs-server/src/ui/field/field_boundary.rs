//! @canon-level: strict
//! Field Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::field_ui::Field as FieldUi;
use super::variants::{
    FieldOrientation,
    FieldValidation
};

#[component]
pub fn Field(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FieldUi orientation=FieldOrientation::Vertical _validation=FieldValidation::None disabled=disabled class=class>
            {children()}
        </FieldUi>
    }
}
