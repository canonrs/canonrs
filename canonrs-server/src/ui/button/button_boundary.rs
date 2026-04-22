//! @canon-level: strict
//! Button Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::button_ui::Button as ButtonUi;
pub use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonType, ButtonStateHint};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] validation: Option<String>,
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] target: String,
    #[prop(default = ButtonType::Button)] button_type: ButtonType,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    let _ = (validation, state_hint);
    view! {
        <ButtonUi
            variant=variant
            size=size
            disabled=disabled_state
            button_type=button_type
            aria_label=aria_label.unwrap_or_default()
            class=class
            href=href
            target=target
        >
            {children()}
        </ButtonUi>
    }
}
