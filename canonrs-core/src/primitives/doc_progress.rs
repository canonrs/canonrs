//! DocProgress Primitive - Progress bar for document reading
//! Pure SSR component, zero state, zero effects

use leptos::prelude::*;

#[component]
pub fn DocProgressPrimitive(
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = "0".to_string())] data_progress: String,
) -> impl IntoView {
    let progress_value = data_progress.clone();
    
    view! {
        <div
            data-doc-progress=""
            id={id}
            class={class}
            data-progress={data_progress}
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow={progress_value}
            aria-label="Reading progress"
        >
            <div data-doc-progress-bar=""></div>
        </div>
    }
}
