use leptos::prelude::*;
use canonrs_core::ToggleState;
use canonrs_core::primitives::{ButtonGroupPrimitive, ButtonPrimitive, ButtonVariant as CoreVariant};

#[component]
pub fn ButtonGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] attached: ToggleState,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonGroupPrimitive class=class attached=attached aria_label=aria_label.unwrap_or_default()>
            {children()}
        </ButtonGroupPrimitive>
    }
}

#[component]
pub fn ButtonGroupPreview() -> impl IntoView {
    view! {
        <ButtonGroup aria_label="Preview group">
            <ButtonPrimitive variant=CoreVariant::Outline>"Left"</ButtonPrimitive>
            <ButtonPrimitive variant=CoreVariant::Outline>"Center"</ButtonPrimitive>
            <ButtonPrimitive variant=CoreVariant::Outline>"Right"</ButtonPrimitive>
        </ButtonGroup>
    }
}
