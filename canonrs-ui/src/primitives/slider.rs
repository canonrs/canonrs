//! @canon-level: strict
//! @canon-owner: primitives-team
//! Slider Primitive - HTML puro + ARIA

use leptos::prelude::*;
use leptos::html;

#[component]
pub fn SliderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] _step: f64,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = "horizontal".to_string())] orientation: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let clamped_value = value.clamp(min, max);
    let percent = ((clamped_value - min) / (max - min)) * 100.0;

    view! {
        <div
            node_ref=node_ref
            data-slider=""
            attr:data-orientation={orientation.clone()}
            attr:data-disabled={if disabled { "true" } else { "" }}
            attr:data-value={clamped_value.to_string()}
            attr:data-percent={percent.to_string()}
            role="slider"
            attr:aria-valuemin={min.to_string()}
            attr:aria-valuemax={max.to_string()}
            attr:aria-valuenow={clamped_value.to_string()}
            attr:aria-orientation={orientation}
            tabindex={if disabled { "-1" } else { "0" }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SliderTrackPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-slider-track=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SliderRangePrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-slider-range=""
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn SliderThumbPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-slider-thumb=""
            class={class}
            id={id}
        />
    }
}
