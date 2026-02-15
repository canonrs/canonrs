use leptos::prelude::*;
use crate::primitives::PulsePrimitive;

pub use crate::primitives::{PulseVariant, PulseSize, PulseSpeed};

#[component]
pub fn Pulse(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = PulseVariant::Default)] variant: PulseVariant,
    #[prop(default = PulseSize::Medium)] size: PulseSize,
    #[prop(default = PulseSpeed::Normal)] speed: PulseSpeed,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <PulsePrimitive variant=variant size=size speed=speed class=class id=id>
            {children.map(|c| c())}
        </PulsePrimitive>
    }
}
