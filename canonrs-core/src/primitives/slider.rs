//! @canon-level: strict
//! @canon-owner: primitives-team
//! Slider Primitive - HTML puro + ARIA

use leptos::prelude::*;
use leptos::html;

#[component]
pub fn SliderPrimitive(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<html::Div>>,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] _step: f64,
    #[prop(default = 0.0)] value: f64,
    #[prop(into, default = "horizontal".to_string())] orientation: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let clamped_value = value.clamp(min, max);
    let percent = ((clamped_value - min) / (max - min)) * 100.0;
    let nr = node_ref.unwrap_or_else(|| NodeRef::new());

    view! {
        <div
            node_ref=nr
            data-rs-slider=""
            data-rs-orientation=orientation.clone()
            data-rs-disabled={if disabled { Some("true") } else { None }}
            data-rs-state={if disabled { "disabled" } else { "closed" }}
            data-rs-value=clamped_value.to_string()
            data-rs-percent=percent.to_string()
            role="slider"
            aria-valuemin=min.to_string()
            aria-valuemax=max.to_string()
            aria-valuenow=clamped_value.to_string()
            aria-orientation=orientation
            aria-disabled={if disabled { Some("true") } else { None }}
            tabindex={if disabled { "-1" } else { "0" }}
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SliderTrackPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-slider-track="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SliderRangePrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-slider-range="" class=class />
    }
}

#[component]
pub fn SliderThumbPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-slider-thumb="" class=class />
    }
}
