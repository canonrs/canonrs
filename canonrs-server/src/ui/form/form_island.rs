use leptos::prelude::*;
use canonrs_core::primitives::FormValidationState;
use crate::ui::form::form_ui::Form;

#[island]
pub fn FormIsland(
    children: Children,
    #[prop(optional, into)] action: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let action = action.unwrap_or_default();
    let class  = class.unwrap_or_default();

    let validation = FormValidationState::Idle;

    view! {
        <Form
            action=action
            validation=validation
            class=class
        >
            {children()}
        </Form>
    }
}
