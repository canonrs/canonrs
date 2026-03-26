//! @canon-level: ui
//! Switch - native checkbox, CSS-driven via :checked

use leptos::prelude::*;
use canonrs_core::primitives::{SwitchPrimitive, SwitchThumbPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn Switch(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let checked_state = if checked { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <SwitchPrimitive
            checked=checked_state
            disabled=disabled_state
            name=name
            value=value
            class=class
        >
            <SwitchThumbPrimitive />
            {children.map(|c| c())}
        </SwitchPrimitive>
    }
}

#[component]
pub fn SwitchPreview() -> impl IntoView {
    view! {
        <Switch />
        <Switch checked=true />
        <Switch disabled=true />
    }
}
