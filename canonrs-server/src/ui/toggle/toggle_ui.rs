use leptos::prelude::*;
use canonrs_core::primitives::TogglePrimitive;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <TogglePrimitive
            class={class}
            id={id.unwrap_or_default()}
            pressed={pressed}
            aria_label=aria_label
        >
            {children()}
        </TogglePrimitive>
    }
}

#[component]
pub fn TogglePreview() -> impl IntoView {
    view! { <Toggle>"Toggle"</Toggle> }
}
