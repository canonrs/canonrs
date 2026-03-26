use leptos::prelude::*;
use canonrs_core::primitives::AnimatePrimitive;
pub use canonrs_core::primitives::{AnimationName, AnimationEasing};

#[component]
pub fn Animate(
    children: Children,
    #[prop(default = AnimationName::FadeIn)] animation: AnimationName,
    #[prop(default = AnimationEasing::EaseInOut)] easing: AnimationEasing,
    #[prop(into, default = String::new())] duration: String,
    #[prop(into, default = String::new())] delay: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AnimatePrimitive
            animation=animation
            easing=easing
            duration=duration
            delay=delay
            class=class
        >
            {children()}
        </AnimatePrimitive>
    }
}

#[component]
pub fn AnimatePreview() -> leptos::prelude::AnyView {
    view! { <Animate>"Content"</Animate> }.into_any()
}
