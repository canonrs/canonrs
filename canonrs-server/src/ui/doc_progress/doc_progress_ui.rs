//! @canon-id: doc-progress
//! @canon-label: Doc Progress
//! @canon-family: utility
//! @canon-category: Display
//! @canon-intent: Indicate reading progress in a document
//! @canon-description: Document progress indicator
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: doc-progress, document, progress, reading, steps

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
