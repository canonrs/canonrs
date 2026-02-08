use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::slider::Slider;
use canonrs_ui::shared::Orientation;

#[component]
pub fn SliderInteractive(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = false)] disabled: bool,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(optional)] on_change: Option<Callback<f64>>,
) -> impl IntoView {
    view! {
        <Slider
            min=min
            max=max
            step=step
            value=value
            disabled=disabled
            orientation=orientation
            class=class
            id=id
            on:input=move |ev: ev::Event| {
                if let Some(ref handler) = on_change {
                    if let Ok(new_value) = event_target_value(&ev).parse::<f64>() {
                        handler.run(new_value.clamp(min, max));
                    }
                }
            }
        />
    }
}
