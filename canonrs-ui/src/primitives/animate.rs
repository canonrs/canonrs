//! @canon-level: strict
//! @canon-owner: primitives-team
//! Animate Primitive - HTML puro + data-attributes de animação

use leptos::prelude::*;

#[component]
pub fn AnimatePrimitive(
    children: Children,
    #[prop(default = String::new())] animation: String,
    #[prop(default = String::new())] duration: String,
    #[prop(default = String::new())] easing: String,
    #[prop(default = String::new())] delay: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    view! {
        <div
            attr:data-animate=""
            attr:data-animation={animation}
            attr:data-duration={duration}
            attr:data-easing={easing}
            attr:data-delay={delay}
            class=class
            id=id
            style={style}
        >
            {children()}
        </div>
    }
}
