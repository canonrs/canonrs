//! @canon-level: strict
//! Icon Primitive - SVG wrapper

use leptos::prelude::*;

#[component]
pub fn IconPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-rs-icon="" class={class} id=id>
        data-rs-uid=crate::infra::uid::generate("ico")
            {children()}
        </span>
    }
}
