//! @canon-id: toggle
//! @canon-label: Toggle
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Toggle a pressed state
//! @canon-description: Toggle button
//! @canon-composable: false
//! @canon-capabilities: Pressed, Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: toggle, activate, button, on, off, press

use leptos::prelude::*;
use canonrs_core::primitives::TogglePrimitive;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <TogglePrimitive class=class pressed=pressed.into() aria_label=aria_label>
            {children()}
        </TogglePrimitive>
    }
}

#[component]
pub fn TogglePreview() -> impl IntoView {
    view! {
        <Toggle>"Toggle"</Toggle>
        <Toggle pressed=true>"Active"</Toggle>
    }
}
