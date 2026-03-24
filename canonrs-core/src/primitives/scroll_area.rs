//! @canon-level: strict
//! @canon-owner: primitives-team
//! ScrollArea Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ScrollArea(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-rs-scroll-area="" id=id class=class>
            {children.map(|c| c())}
        </div>
    }
}
