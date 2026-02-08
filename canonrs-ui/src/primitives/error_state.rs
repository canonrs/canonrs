//! @canon-level: strict
//! @canon-owner: primitives-team
//! ErrorState Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ErrorStatePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-error-state=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
