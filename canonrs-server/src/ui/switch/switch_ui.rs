//! @canon-level: ui
//! Switch - native checkbox, CSS-driven via :checked

use leptos::prelude::*;
use canonrs_core::primitives::{SwitchPrimitive, SwitchThumbPrimitive};

#[component]
pub fn Switch(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SwitchPrimitive
            checked=checked
            disabled=disabled
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
