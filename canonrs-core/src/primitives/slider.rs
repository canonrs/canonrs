//! @canon-level: strict
//! @canon-owner: primitives-team
//! Slider Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

#[component]
pub fn SliderPrimitive(
    children: Children,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 0.0)] value: f64,
    #[prop(into, default = "horizontal".to_string())] orientation: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let clamped_value = value.clamp(min, max);
    let percent = ((clamped_value - min) / (max - min)) * 100.0;
    view! {
        <div
            data-rs-slider=""
            data-rs-component="Slider"
            data-rs-behavior="selection"
            data-rs-orientation=orientation.clone()
            data-rs-disabled=d.data_rs_disabled
            data-rs-value=clamped_value.to_string()
            data-rs-percent=percent.to_string()
            data-rs-step=step.to_string()
            role="slider"
            aria-valuemin=min.to_string()
            aria-valuemax=max.to_string()
            aria-valuenow=clamped_value.to_string()
            aria-orientation=orientation
            aria-disabled=d.aria_disabled
            tabindex=if disabled == DisabledState::Disabled { "-1" } else { "0" }
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
        <div data-rs-slider-thumb="" tabindex="0" class=class />
    }
}
