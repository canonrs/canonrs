//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SliderSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn SliderPrimitive(
    #[prop(default = SliderSize::Md)] size: SliderSize,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(into)] value: RwSignal<f64>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <input
            type="range"
            min=min
            max=max
            step=step
            prop:value=move || value.get()
            on:input=move |ev| {
                if let Ok(new_value) = event_target_value(&ev).parse::<f64>() {
                    value.set(new_value);
                }
            }
            disabled=disabled
            data-size=match size {
                SliderSize::Sm => "sm",
                SliderSize::Md => "md",
                SliderSize::Lg => "lg",
            }
        />
    }
}
