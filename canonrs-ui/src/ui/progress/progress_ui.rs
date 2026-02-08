use leptos::prelude::*;
use crate::primitives::{ProgressPrimitive, ProgressIndicatorPrimitive};

#[component]
pub fn Progress(
    #[prop(default = 0.0)] value: f64,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let clamped = value.clamp(0.0, 100.0);
    let transform_style = format!("transform: translateX(-{}%)", 100.0 - clamped);
    let base_class = format!("progress {}", class);

    view! {
        <ProgressPrimitive
            value={clamped}
            class={base_class}
            id={id}
        >
            <ProgressIndicatorPrimitive
                class="progress-indicator".to_string()
                style={transform_style}
            />
        </ProgressPrimitive>
    }
}
