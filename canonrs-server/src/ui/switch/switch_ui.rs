
use leptos::prelude::*;
use canonrs_core::primitives::{SwitchPrimitive, SwitchThumbPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn Switch(
    children: Children,
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
            {children()}
        </SwitchPrimitive>
    }
}

#[component]
pub fn SwitchPreview() -> impl IntoView {
    view! {
        <Switch>"Off"</Switch>
        <Switch checked=true>"On"</Switch>
        <Switch disabled=true>"Disabled"</Switch>
    }
}
