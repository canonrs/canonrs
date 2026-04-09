use leptos::prelude::*;
use canonrs_core::primitives::{ButtonPrimitive, ButtonVariant, ButtonSize};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            class=class
            disabled=disabled
            aria_label=aria_label.unwrap_or_default()
            variant=variant
            size=size
        >
            {children()}
        </ButtonPrimitive>
    }
}

#[component]
pub fn ButtonPreview() -> impl IntoView {
    view! {
        <Button>"Primary"</Button>
        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
        <Button variant=ButtonVariant::Outline>"Outline"</Button>
        <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
        <Button disabled=DisabledState::Disabled>"Disabled"</Button>
    }
}
