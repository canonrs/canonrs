#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationVariant {
    FadeIn,
    SlideIn,
    Bounce,
    Shake,
    FadeUp,
}

impl AnimationVariant {
    pub fn to_class(&self) -> &'static str {
        match self {
            Self::FadeIn => "animate-fade-in",
            Self::SlideIn => "animate-slide-in",
            Self::Bounce => "animate-bounce",
            Self::Shake => "animate-shake",
            Self::FadeUp => "animate-fade-up",
        }
    }
}

// Animation tokens struct
pub struct AnimationTokens {
    pub duration: &'static str,
    pub easing: &'static str,
}

impl AnimationTokens {
    pub const DURATION_BASE: &'static str = "300ms";
    pub const DELAY_NONE: &'static str = "0ms";
}

pub const ANIMATION_TOKENS: AnimationTokens = AnimationTokens {
    duration: "300ms",
    easing: "cubic-bezier(0.4, 0, 0.2, 1)",
};
