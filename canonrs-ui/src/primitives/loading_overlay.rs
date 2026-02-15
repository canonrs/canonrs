//! @canon-level: strict
//! @canon-owner: primitives-team
//! LoadingOverlay Primitive - Visual loading wrapper

use leptos::prelude::*;

#[component]
pub fn LoadingOverlayPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] loading: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-loading-overlay=""
            data-loading={if loading { "true" } else { "false" }}
            aria-busy={if loading { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
