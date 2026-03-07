//! @canon-level: strict
//! Icon Primitive - SVG wrapper

use leptos::prelude::*;

#[component]
pub fn IconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span data-icon="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children.map(|c| c())}
        </span>
    }
}
