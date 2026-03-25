//! @canon-level: strict
//! @canon-owner: primitives-team
//! ScrollArea Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ScrollArea(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div data-rs-scroll-area="" id=if id.is_empty() { None } else { Some(id.clone()) } class=class>
            {children()}
        </div>
    }
}
