//! @canon-level: strict
//! @canon-owner: primitives-team
//! AspectRatio Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn AspectRatioPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::from("16/9"))] ratio: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-aspect-ratio=""
            data-ratio={ratio}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            <div data-rs-aspect-content="">
                {children.map(|c| c())}
            </div>
        </div>
    }
}
