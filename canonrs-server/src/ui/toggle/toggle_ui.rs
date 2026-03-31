
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
