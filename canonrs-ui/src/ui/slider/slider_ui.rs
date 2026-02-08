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
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let clamped = value.clamp(min, max);
    let node_ref = NodeRef::<html::Div>::new();

    let base_class = format!(
        "slider orientation-{} {}",
        orientation.as_str(),
        class
    );

    view! {
        <SliderPrimitive
            node_ref=node_ref
            min={min}
            max={max}
            _step={step}
            value={clamped}
            orientation={orientation.as_str().to_string()}
            disabled={disabled}
            class={base_class}
            id={id}
        >
            <SliderTrackPrimitive class="slider-track".to_string()>
                <SliderRangePrimitive class="slider-range".to_string() />
            </SliderTrackPrimitive>
            <SliderThumbPrimitive class="slider-thumb".to_string() />
        </SliderPrimitive>
    }
}
