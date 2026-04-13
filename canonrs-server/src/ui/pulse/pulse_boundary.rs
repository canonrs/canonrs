//! Pulse Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
pub use super::pulse_ui::{PulseVariant, PulseSize, PulseSpeed};
use super::pulse_ui::Pulse as PulseUi;

#[component]
pub fn Pulse(
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size:         PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed:       PulseSpeed,
    #[prop(into, default = String::new())] class:      String,
) -> impl IntoView {
    view! { <PulseUi variant=variant size=size speed=speed class=class>{""}</PulseUi> }
}
