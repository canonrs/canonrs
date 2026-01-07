//! Animate UI Component - Enterprise animations

use crate::primitives::AnimatePrimitive;
use crate::tokens::animations::{AnimationTokens, AnimationVariant};
use leptos::prelude::*;

#[component]
pub fn Animate(
    #[prop(optional)] variant: Option<AnimationVariant>,
    #[prop(optional)] duration: Option<&'static str>,
    #[prop(optional)] delay: Option<&'static str>,
    #[prop(optional)] hover: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let animation_class = variant.map(|v| v.to_class()).unwrap_or("");
    let duration_val = duration.unwrap_or(AnimationTokens::DURATION_BASE);
    let delay_val = delay.unwrap_or(AnimationTokens::DELAY_NONE);

    let merged_class = format!(
        "{} {}",
        class,
        if hover {
            format!("hover:{}", animation_class)
        } else {
            animation_class.to_string()
        }
    );

    view! {
        <AnimatePrimitive
            class=merged_class
            data_animation=variant.map(|v| v.to_class().to_string()).unwrap_or_default()
            data_duration=duration_val.to_string()
            data_delay=delay_val.to_string()
        >
            {children()}
        </AnimatePrimitive>
    }
}

// Helper para animações em hover
#[component]
pub fn AnimateOnHover(
    #[prop(optional)] variant: Option<AnimationVariant>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <Animate variant=variant.unwrap_or(AnimationVariant::FadeIn) hover=true class=class>
            {children()}
        </Animate>
    }
}
