//! @canon-level: strict
//! @canon-owner: primitives-team
//! DocProgress Primitive - Progress bar for document reading
//! Pure SSR component, zero state, zero effects

use leptos::prelude::*;

#[component]
pub fn DocProgressPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 0u8)] progress: u8,
) -> impl IntoView {
    let progress_str = progress.to_string();
    view! {
        <div
            data-rs-doc-progress=""
            data-rs-progress=progress_str.clone()
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=progress_str
            aria-label="Reading progress"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            <div data-rs-doc-progress-bar="" />
        </div>
    }
}
