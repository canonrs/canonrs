//! @canon-level: strict
//! @canon-owner: primitives-team
//! ToggleGroup Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ToggleGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-toggle-group=""
            data-rs-multiple=if multiple { "true" } else { "false" }
            role="group"
            class=class
            id=if id.is_empty() { None } else { Some(id) }
        >
            {children()}
        </div>
    }
}
