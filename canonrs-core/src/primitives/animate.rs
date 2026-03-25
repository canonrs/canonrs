//! @canon-level: strict
//! @canon-owner: primitives-team
//! Animate Primitive - HTML puro + data-attributes de animacao

use leptos::prelude::*;

#[component]
pub fn AnimatePrimitive(
    children: Children,
    #[prop(into, default = String::new())] animation: String,
    #[prop(into, default = String::new())] duration: String,
    #[prop(into, default = String::new())] easing: String,
    #[prop(into, default = String::new())] delay: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-animate=""
            data-rs-animation=animation
            data-rs-duration=duration
            data-rs-easing=easing
            data-rs-delay=delay
            class=class
            id=id.filter(|s| !s.is_empty())
            style=style.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
