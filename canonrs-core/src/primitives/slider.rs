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
    let safe_value = if value.is_nan() { min } else { value };
    let safe_min = if min.is_nan() { 0.0 } else { min };
    let safe_max = if max.is_nan() || max <= safe_min { safe_min + 100.0 } else { max };
    let clamped_value = safe_value.clamp(safe_min, safe_max);
    let percent = ((clamped_value - safe_min) / (safe_max - safe_min)) * 100.0;
    view! {
        <div
            data-rs-slider=""
            data-rs-component="Slider"
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
        <div data-rs-slider-track="" data-rs-component="SliderTrack" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SliderRangePrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-slider-range="" data-rs-component="SliderRange" class=class />
    }
}

#[component]
pub fn SliderThumbPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-slider-thumb="" data-rs-component="SliderThumb" tabindex="0" aria-label="Slider thumb" class=class />
    }
}

#[component]
pub fn SliderMarksPrimitive(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 10.0)] step: f64,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let safe_step = if step <= 0.0 { 10.0 } else { step };
    let count = ((max - min) / safe_step).round() as usize + 1;
    let marks: Vec<f64> = (0..count)
        .map(|i| min + i as f64 * safe_step)
        .filter(|v| *v <= max + 1e-9)
        .collect();
    view! {
        <div data-rs-slider-marks="" class=class>
            {marks.into_iter().map(|v| {
                let pct = ((v - min) / (max - min)) * 100.0;
                let style = format!("left: {:.4}%", pct);
                view! { <span data-rs-slider-mark="" style=style /> }
            }).collect::<Vec<_>>()}
        </div>
    }
}
