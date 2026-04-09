//! Slider Island — Canon Rule passthrough
use leptos::prelude::*;
use super::slider_ui::{Slider, SliderWithMarks};
use canonrs_core::meta::DisabledState;
use canonrs_core::Orientation;

#[component]
pub fn SliderIsland(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Slider min=min max=max step=step value=value disabled=disabled orientation=orientation class=class />
    }
}

#[component]
pub fn SliderWithMarksIsland(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 10.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SliderWithMarks min=min max=max step=step value=value disabled=disabled orientation=orientation class=class />
    }
}
