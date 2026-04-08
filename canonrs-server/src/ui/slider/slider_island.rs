//! @canon-level: strict
//! Slider Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::slider_ui::{Slider, SliderWithMarks};
use canonrs_core::meta::DisabledState;
use canonrs_core::Orientation;

#[island]
pub fn SliderInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::slider::init_all();
        });
    }
    view! { <></> }
}

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
        <SliderInit />
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
        <SliderInit />
        <SliderWithMarks min=min max=max step=step value=value disabled=disabled orientation=orientation class=class />
    }
}
