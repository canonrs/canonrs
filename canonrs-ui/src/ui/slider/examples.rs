use leptos::prelude::*;
use super::Slider;
use crate::shared::Orientation;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Slider min=0.0 max=100.0 value=50.0 />
    }
}

#[component]
pub fn range_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
                <label>"0-10 range"</label>
                <Slider min=0.0 max=10.0 value=5.0 step=1.0 />
            </div>
            <div>
                <label>"0-1000 range"</label>
                <Slider min=0.0 max=1000.0 value=500.0 step=10.0 />
            </div>
        </div>
    }
}

#[component]
pub fn orientation_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 2rem;">
            <div>
                <label>"Horizontal"</label>
                <Slider min=0.0 max=100.0 value=50.0 orientation=Orientation::Horizontal />
            </div>
            <div>
                <label>"Vertical"</label>
                <Slider min=0.0 max=100.0 value=50.0 orientation=Orientation::Vertical />
            </div>
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <Slider min=0.0 max=100.0 value=30.0 disabled=true />
    }
}
