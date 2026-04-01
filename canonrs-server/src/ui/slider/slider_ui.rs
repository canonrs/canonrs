
use leptos::prelude::*;
use canonrs_core::Orientation;
use canonrs_core::primitives::{
    SliderPrimitive, SliderTrackPrimitive,
    SliderRangePrimitive, SliderThumbPrimitive,
};
use canonrs_core::meta::DisabledState;

#[component]
pub fn Slider(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SliderPrimitive
            min={min}
            max={max}
            step={step}
            value={value}
            orientation={orientation.as_str().to_string()}
            disabled=disabled
            class={class}
        >
            <SliderTrackPrimitive>
                <SliderRangePrimitive />
                <SliderThumbPrimitive />
            </SliderTrackPrimitive>
        </SliderPrimitive>
    }
}

#[component]
pub fn SliderPreview() -> impl IntoView {
    view! { <Slider min=0.0 max=100.0 value=50.0 /> }
}
