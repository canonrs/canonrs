//! @canon-level: strict
//! Form Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use canonrs_core::primitives::FormValidationState;
use crate::ui::form::form_ui::Form as FormUi;

#[component]
pub fn Form(
    children: Children,
    #[prop(into, default = String::new())] action: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <FormUi action=action validation=FormValidationState::Idle class=class>
            {children()
};
        </FormUi>
    }
}
