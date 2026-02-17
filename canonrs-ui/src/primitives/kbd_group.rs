//! @canon-level: strict
//! KbdGroup Primitive - Wrapper for keyboard shortcuts

use leptos::prelude::*;

#[component]
pub fn KbdGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-kbd-group=""
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn KbdSeparatorPrimitive() -> impl IntoView {
    view! {
        <span data-kbd-separator="">"+"</span>
    }
}
