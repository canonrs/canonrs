//! @canon-level: ui
//! Pulse - Declarative UI wrapper

use leptos::prelude::*;
use canonrs_core::primitives::PulsePrimitive;
pub use canonrs_core::primitives::{PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn Pulse(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size: PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed: PulseSpeed,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <PulsePrimitive
            variant=variant
            size=size
            speed=speed
            class={class.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </PulsePrimitive>
    }
}

#[component]
pub fn PulsePreview() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:1.5rem;">
            <Pulse variant=PulseVariant::Subtle />
            <Pulse variant=PulseVariant::Default />
            <Pulse variant=PulseVariant::Emphasized />
        </div>
    }
}
