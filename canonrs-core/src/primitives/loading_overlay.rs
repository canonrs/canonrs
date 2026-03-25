//! @canon-level: strict
//! @canon-owner: primitives-team
//! LoadingOverlay Primitive - Visual loading wrapper

use leptos::prelude::*;

#[component]
pub fn LoadingOverlayPrimitive(
    children: Children,
    #[prop(default = false)] loading: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if loading { "loading" } else { "idle" };
    view! {
        <div
            data-rs-loading-overlay=""
            data-rs-state=state
            aria-busy=if loading { Some("true") } else { None }
            class=class
        >
            {children()}
        </div>
    }
}
