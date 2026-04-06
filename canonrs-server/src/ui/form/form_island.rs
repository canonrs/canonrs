use leptos::prelude::*;
use canonrs_core::primitives::FormValidationState;
use crate::ui::form::form_ui::Form;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum FormIslandState {
    #[default]
    Idle,
    Submitting,
    Success,
    Error,
}

impl FormIslandState {
    fn to_validation(self) -> FormValidationState {
        match self {
            Self::Idle       => FormValidationState::Idle,
            Self::Submitting => FormValidationState::Submitting,
            Self::Success    => FormValidationState::Success,
            Self::Error      => FormValidationState::Error,
        }
    }
}

#[island]
pub fn FormIsland(
    children: Children,
    #[prop(optional, into)] action: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let action = action.unwrap_or_default();
    let class  = class.unwrap_or_default();

    let state = RwSignal::new(FormIslandState::Idle);

    let validation = move || state.get().to_validation();

    view! {
        <Form
            action=action
            validation=validation()
            class=class
        >
            {children()}
        </Form>
    }
}
