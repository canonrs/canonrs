//! @canon-level: strict
//! @canon-owner: primitives-team
//! Progress Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ProgressPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0.0)] value: f64,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    let clamped_value = value.clamp(0.0, 100.0);

    view! {
        <div
            data-progress=""
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=clamped_value
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ProgressIndicatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <div
            data-progress-indicator=""
            class=class
            id=id
            style=style
        />
    }
}
