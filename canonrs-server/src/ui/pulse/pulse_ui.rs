//! @canon-id: pulse
//! @canon-label: Pulse
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Animated attention indicator
//! @canon-description: Pulse animation wrapper
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: pulse, animation, glow, attention, highlight, ping

use leptos::prelude::*;
use canonrs_core::primitives::PulsePrimitive;
pub use canonrs_core::primitives::{PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn Pulse(
    children: Children,
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size: PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed: PulseSpeed,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PulsePrimitive
            variant=variant
            size=size
            speed=speed
            class=class
        >
            {children()}
        </PulsePrimitive>
    }
}

#[component]
pub fn PulsePreview() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Pulse variant=PulseVariant::Subtle>"·"</Pulse>
            <Pulse variant=PulseVariant::Default>"·"</Pulse>
            <Pulse variant=PulseVariant::Emphasized>"·"</Pulse>
        </div>
    }
}
