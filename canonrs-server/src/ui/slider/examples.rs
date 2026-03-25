use leptos::prelude::*;
use super::Slider;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Slider min=0.0 max=100.0 value=50.0 />
    }
}
