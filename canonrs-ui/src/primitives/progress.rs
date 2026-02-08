//! @canon-level: strict
//! @canon-owner: primitives-team
//! Progress Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ProgressPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let clamped_value = value.clamp(0.0, 100.0);

    view! {
        <div
            data-progress=""
            role="progressbar"
            attr:aria-valuemin="0"
            attr:aria-valuemax="100"
            attr:aria-valuenow={clamped_value.to_string()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ProgressIndicatorPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <div
            data-progress-indicator=""
            class={class}
            id={id}
            style={style}
        />
    }
}
