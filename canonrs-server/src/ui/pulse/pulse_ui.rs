#![allow(unreachable_pub, dead_code)]

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

