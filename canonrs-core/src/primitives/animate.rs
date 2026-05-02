//! @canon-level: strict
//! @canon-owner: primitives-team
//! Animate Primitive - HTML puro + data-attributes de animacao

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum AnimationName {
    #[default]
    None,
    FadeIn,
    FadeOut,
    SlideIn,
    SlideOut,
    ScaleIn,
    ScaleOut,
}
impl AnimationName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None     => "none",
            Self::FadeIn   => "fade-in",
            Self::FadeOut  => "fade-out",
            Self::SlideIn  => "slide-in",
            Self::SlideOut => "slide-out",
            Self::ScaleIn  => "scale-in",
            Self::ScaleOut => "scale-out",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum AnimationEasing {
    #[default]
    EaseInOut,
    EaseIn,
    EaseOut,
    Linear,
}
impl AnimationEasing {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EaseInOut => "ease-in-out",
            Self::EaseIn    => "ease-in",
            Self::EaseOut   => "ease-out",
            Self::Linear    => "linear",
        }
    }
}

#[component]
pub fn AnimatePrimitive(
    children: Children,
    #[prop(default = AnimationName::None)] animation: AnimationName,
    #[prop(default = AnimationEasing::EaseInOut)] easing: AnimationEasing,
    #[prop(into, default = String::new())] duration: String,
    #[prop(into, default = String::new())] delay: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_an = crate::infra::uid::generate("an");
    view! {
        <div
            data-rs-animate=""
            data-rs-uid=uid_an
            data-rs-interaction="init"
            data-rs-animation=animation.as_str()
            data-rs-easing=easing.as_str()
            data-rs-duration=duration
            data-rs-delay=delay
            class=class
        >
            {children()}
        </div>
    }
}
