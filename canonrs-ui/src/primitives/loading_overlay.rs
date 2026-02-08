//! @canon-level: strict
//! @canon-owner: primitives-team
//! LoadingOverlay Primitive - Visual loading wrapper

use leptos::prelude::*;

#[component]
pub fn LoadingOverlayPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] loading: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-loading-overlay=""
            attr:aria-busy={if loading { "true" } else { "false" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
