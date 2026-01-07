//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

//! Animate Primitive - HTML puro + data-attributes

use leptos::prelude::*;

#[component]
pub fn AnimatePrimitive(
    #[prop(optional)] data_animation: Option<String>,
    #[prop(optional)] data_duration: Option<String>,
    #[prop(optional)] data_delay: Option<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-animation=data_animation
            data-duration=data_duration
            data-delay=data_delay
        >
            {children()}
        </div>
    }
}
