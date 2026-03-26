//! DocProgress - Reading progress indicator
//! SSR-safe, behavior-driven

use leptos::prelude::*;
use canonrs_core::primitives::doc_progress::DocProgressPrimitive;

#[component]
pub fn DocProgress(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DocProgressPrimitive
            class=class
            progress=0u8
        />
    }
}
