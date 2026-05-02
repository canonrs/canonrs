//! @canon-level: strict
//! Button Boundary — Tipo 2: Init
//! Normaliza props (bool -> State), delega para canonrs-interactions-init

use leptos::prelude::*;
use super::button_ui::{Button as ButtonUi, LinkButton as LinkButtonUi};
pub use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonType, ButtonStateHint};
use canonrs_core::meta::{DisabledState, LoadingState};

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] validation: Option<String>,
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
    #[prop(default = ButtonType::Button)] button_type: ButtonType,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    let loading_state  = if loading  { LoadingState::Loading  } else { LoadingState::Idle };
    let _ = (validation, state_hint);
    view! {
        <ButtonUi
            variant=variant
            size=size
            disabled=disabled_state
            loading=loading_state
            button_type=button_type
            aria_label=aria_label.unwrap_or_default()
            class=class
        >
            {children()}
        </ButtonUi>
    }
}

#[component]
pub fn LinkButton(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] target: String,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <LinkButtonUi
            variant=variant
            size=size
            disabled=disabled_state
            aria_label=aria_label.unwrap_or_default()
            href=href
            target=target
            class=class
        >
            {children()}
        </LinkButtonUi>
    }
}
