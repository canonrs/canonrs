use leptos::prelude::*;
use crate::primitives::SpinnerPrimitive;

pub use crate::primitives::{SpinnerSize};

#[component]
pub fn Spinner(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(into, default = String::new())] custom_size: String,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    let style = if custom_size.is_empty() {
        String::new()
    } else {
        format!("width: {}; height: {};", custom_size, custom_size)
    };

    view! {
        <SpinnerPrimitive
            size=size
            paused=paused
            aria_label=aria_label
            style=style
            class=class
            id=id
        />
    }
}
