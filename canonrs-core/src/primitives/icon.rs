//! @canon-level: strict
//! Icon Primitive - SVG wrapper

use leptos::prelude::*;

#[component]
pub fn IconPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span data-rs-icon="" data-rs-component="Icon" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children()}
        </span>
    }
}
