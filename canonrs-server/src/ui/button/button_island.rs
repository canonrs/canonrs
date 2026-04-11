//! @canon-level: strict
//! Button Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::button_ui::Button;
use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonType, ButtonStateHint};
use canonrs_core::meta::DisabledState;

#[component]
pub fn ButtonIsland(
    children: Children,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] validation: Option<String>,
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    let _ = (validation, state_hint); // passed via data-rs-state by init module
    view! {
        <Button
            variant=variant
            size=size
            disabled=disabled_state
            button_type=ButtonType::Button
            aria_label=aria_label.unwrap_or_default()
            class=class
        >
            {children()}
        </Button>
    }
}
