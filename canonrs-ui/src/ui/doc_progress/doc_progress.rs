//! DocProgress - Reading progress indicator
//! SSR-safe, behavior-driven

use leptos::prelude::*;
use crate::primitives::doc_progress::DocProgressPrimitive;

#[component]
pub fn DocProgress(
    #[prop(into, default = "doc-progress".to_string())] id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DocProgressPrimitive
            id=id
            class=class
            data_progress="0".to_string()
        />
    }
}
