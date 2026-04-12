//! @canon-level: strict
//! Animate Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::animate_ui::{
    Animate as AnimateUi,
    AnimationName,
    AnimationEasing
};

#[component]
pub fn Animate(
    children: Children,
    #[prop(into, default = String::from("fade-in"))] animation: String,
    #[prop(into, default = String::from("ease-in-out"))] easing: String,
    #[prop(into, default = String::from("300ms"))] duration: String,
    #[prop(into, default = String::new())] delay: String,
    #[prop(optional)] stagger: Option<f64>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let animation_val = match animation.as_str() {
        "fade-out"  => AnimationName::FadeOut,
        "slide-in"  => AnimationName::SlideIn,
        "slide-out" => AnimationName::SlideOut,
        "scale-in"  => AnimationName::ScaleIn,
        "scale-out" => AnimationName::ScaleOut,
        _           => AnimationName::FadeIn,
    };
    let easing_val = match easing.as_str() {
        "ease-in"  => AnimationEasing::EaseIn,
        "ease-out" => AnimationEasing::EaseOut,
        "linear"   => AnimationEasing::Linear,
        _          => AnimationEasing::EaseInOut,
    };
    let _ = stagger; // handled by init module
    view! {
        <AnimateUi animation=animation_val easing=easing_val duration=duration delay=delay class=class>
            {children()}
        </AnimateUi>
    }
}
