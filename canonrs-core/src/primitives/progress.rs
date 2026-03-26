//! @canon-level: strict
//! @canon-owner: primitives-team
//! Progress Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ProgressPrimitive(
    children: Children,
    #[prop(default = 0.0)] value: f64,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let clamped = value.clamp(0.0, 100.0);
    view! {
        <div
            data-rs-progress=""
            data-rs-component="Progress"
            data-rs-behavior="feedback"
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=clamped
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ProgressIndicatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-progress-indicator=""
            class=class
            style=style
        />
    }
}
