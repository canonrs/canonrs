//! @canon-level: strict
//! InputGroup Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn InputGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] merge_radius: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-input-group=""
            data-rs-merge-radius={if merge_radius { Some("") } else { None }}
            class=class
        >
            {children()}
        </div>
    }
}
