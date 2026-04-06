use leptos::prelude::*;
use super::pulse_ui::{Pulse, PulseVariant, PulseSize, PulseSpeed};

#[island]
pub fn PulseIsland(
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] speed: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let variant = match variant.as_deref() {
        Some("subtle")     => PulseVariant::Subtle,
        Some("emphasized") => PulseVariant::Emphasized,
        _                  => PulseVariant::Default,
    };
    let size = match size.as_deref() {
        Some("small") => PulseSize::Small,
        Some("large") => PulseSize::Large,
        _             => PulseSize::Medium,
    };
    let speed = match speed.as_deref() {
        Some("slow") => PulseSpeed::Slow,
        Some("fast") => PulseSpeed::Fast,
        _            => PulseSpeed::Normal,
    };
    let cls = class.unwrap_or_default();

    view! {
        <Pulse variant=variant size=size speed=speed class=cls>
            ""
        </Pulse>
    }
}
