use leptos::prelude::*;
use crate::primitives::TogglePrimitive;

#[component]
pub fn Toggle(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <TogglePrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </TogglePrimitive>
    }
}
