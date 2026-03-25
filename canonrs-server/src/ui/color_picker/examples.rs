use leptos::prelude::*;
use super::ColorPicker;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div data-rs-color-picker-example="">
            <ColorPicker value="#3b82f6" />
        </div>
    }
}
