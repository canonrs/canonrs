//! Pulse Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::pulse_ui::{Pulse, PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn PulseIsland(
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size:         PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed:       PulseSpeed,
    #[prop(into, default = String::new())] class:      String,
) -> impl IntoView {
    view! { <Pulse variant=variant size=size speed=speed class=class>{""}</Pulse> }
}
