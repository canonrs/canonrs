//! Slider Island — Canon Rule passthrough
use leptos::prelude::*;
use super::slider_ui::{
    Slider as SliderUi,
    SliderWithMarks as SliderWithMarksUi
};
use canonrs_core::meta::DisabledState;
use canonrs_core::Orientation;

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
        <SliderUi min=min max=max step=step value=value disabled=disabled orientation=orientation class=class />
    }
}

#[component]
pub fn SliderWithMarks(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 10.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SliderWithMarksUi min=min max=max step=step value=value disabled=disabled orientation=orientation class=class />
    }
}
