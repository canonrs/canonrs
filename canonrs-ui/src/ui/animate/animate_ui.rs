use leptos::prelude::*;
use crate::primitives::AnimatePrimitive;

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn,
    SlideOut,
    ScaleIn,
    ScaleOut,
}

impl AnimationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimationType::FadeIn => "fade-in",
            AnimationType::FadeOut => "fade-out",
            AnimationType::SlideIn => "slide-in",
            AnimationType::SlideOut => "slide-out",
            AnimationType::ScaleIn => "scale-in",
            AnimationType::ScaleOut => "scale-out",
        }
    }
}

#[component]
pub fn Animate(
    children: Children,
    #[prop(default = AnimationType::FadeIn)] animation_type: AnimationType,
    #[prop(optional)] duration: Option<String>,
    #[prop(optional)] easing: Option<String>,
    #[prop(optional)] delay: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AnimatePrimitive
            animation={animation_type.as_str().to_string()}
            duration={duration.unwrap_or_default()}
            easing={easing.unwrap_or_default()}
            delay={delay.unwrap_or_default()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </AnimatePrimitive>
    }
}
