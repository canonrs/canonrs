//! @canon-level: strict
//! KbdGroup Primitive - Wrapper for keyboard shortcuts

use leptos::prelude::*;

#[component]
pub fn KbdGroupPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let uid_kbg = crate::infra::uid::generate("kbg");
    view! {
        <span
            data-rs-kbd-group=""
            data-rs-uid=uid_kbg
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </span>
    }
}

#[component]
pub fn KbdSeparatorPrimitive() -> impl IntoView {
    view! {
        <span data-rs-kbd-separator="">"+"</span>
    }
}
