use leptos::prelude::*;
use leptos::html;
use crate::shared::Orientation;
use crate::primitives::{
    SliderPrimitive,
    SliderTrackPrimitive,
    SliderRangePrimitive,
    SliderThumbPrimitive,
};

#[component]
pub fn Slider(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = false)] disabled: bool,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let clamped = value.clamp(min, max);
    let node_ref = NodeRef::<html::Div>::new();

    view! {
        <SliderPrimitive
            node_ref=node_ref
            min={min}
            max={max}
            _step={step}
            value={clamped}
            orientation={orientation.as_str().to_string()}
            disabled={disabled}
            class={class}
            id={id.unwrap_or_default()}
        >
            <SliderTrackPrimitive>
                <SliderRangePrimitive />
            </SliderTrackPrimitive>
            <SliderThumbPrimitive />
        </SliderPrimitive>
    }
}
