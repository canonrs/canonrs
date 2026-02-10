use leptos::prelude::*;
use crate::primitives::toggle::TogglePrimitive;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(default = false)] pressed: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <TogglePrimitive
            pressed={pressed}
            disabled={disabled}
            aria_label={aria_label.unwrap_or_default()}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </TogglePrimitive>
    }
}
