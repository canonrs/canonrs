use leptos::prelude::*;
use canonrs_core::primitives::{ButtonPrimitive, LinkButtonPrimitive, ButtonVariant, ButtonSize, ButtonType};
use canonrs_core::meta::{DisabledState, LoadingState};

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(default = ButtonType::Button)] button_type: ButtonType,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            variant=variant
            size=size
            disabled=disabled
            loading=loading
            button_type=button_type
            aria_label=aria_label.unwrap_or_default()
            class=class
        >
            {children()}
        </ButtonPrimitive>
    }
}

#[component]
pub fn LinkButton(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] target: String,
) -> impl IntoView {
    view! {
        <LinkButtonPrimitive
            href=href
            target=target
            variant=variant
            size=size
            disabled=disabled
            aria_label=aria_label.unwrap_or_default()
            class=class
        >
            {children()}
        </LinkButtonPrimitive>
    }
}
