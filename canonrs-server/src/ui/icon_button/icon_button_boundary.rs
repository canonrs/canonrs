//! @canon-level: strict
//! IconButton Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::icon_button_ui::IconButton as IconButtonUi;
use canonrs_core::primitives::{
    IconButtonVariant,
    IconButtonSize
};
use canonrs_core::meta::{DisabledState, LoadingState, ToggleState};

#[component]
pub fn IconButton(
    children: Children,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(default = IconButtonVariant::Default)] variant: IconButtonVariant,
    #[prop(default = IconButtonSize::Md)] size: IconButtonSize,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] pressed: bool,
    #[prop(default = false)] loading: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    let loading_state  = if loading  { LoadingState::Loading }   else { LoadingState::Idle };
    let pressed_state  = if pressed  { ToggleState::On }         else { ToggleState::Off };
    view! {
        <IconButtonUi
            aria_label=aria_label
            variant=variant
            size=size
            disabled=disabled_state
            loading=loading_state
            pressed=pressed_state
            class=class
        >
            {children()}
        </IconButtonUi>
    }
}
