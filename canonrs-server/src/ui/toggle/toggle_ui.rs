#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::TogglePrimitive;

#[component]
pub fn Toggle(children: Children, #[prop(into, default = String::new())] class: String, #[prop(default = false)] pressed: bool, #[prop(default = false)] disabled: bool, #[prop(into, default = String::new())] aria_label: String) -> impl IntoView {
    view! {
        <TogglePrimitive class=class pressed=pressed.into() disabled=disabled.into() aria_label=aria_label>
            {children()}
        </TogglePrimitive>
    }
}
