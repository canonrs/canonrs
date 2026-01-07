use crate::primitives::{SliderPrimitive, SliderSize as PrimitiveSliderSize};
use leptos::prelude::*;

#[component]
pub fn Slider(
    #[prop(default = PrimitiveSliderSize::Md)] size: PrimitiveSliderSize,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(into)] value: RwSignal<f64>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <SliderPrimitive
            size=size
            min=min
            max=max
            step=step
            value=value
            disabled=disabled
        />
    }
}
