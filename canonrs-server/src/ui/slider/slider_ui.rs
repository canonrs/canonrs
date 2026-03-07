use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::closure::Closure;
use leptos::html;
use canonrs_core::shared::Orientation;
use canonrs_core::primitives::{
    SliderPrimitive,
    SliderTrackPrimitive,
    SliderRangePrimitive,
    SliderThumbPrimitive,
};

#[component]
pub fn Slider(
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(default = 50.0)] value: f64,
    #[prop(default = false)] disabled: bool,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] on_change: Option<Callback<f64>>,
) -> impl IntoView {
    let clamped = value.clamp(min, max);
    let node_ref = NodeRef::<html::Div>::new();

    if on_change.is_some() {
        leptos::prelude::Effect::new(move |_| {
            let Some(el) = node_ref.get() else { return };
            use leptos::wasm_bindgen::JsCast;
            let html_el = el.unchecked_ref::<leptos::web_sys::HtmlElement>();
            let cb = on_change.clone();
            let closure = Closure::wrap(Box::new(move |e: leptos::web_sys::CustomEvent| {
                if let Some(cb) = &cb {
                    let val = e.detail().as_f64().unwrap_or(0.0);
                    cb.run(val);
                }
            }) as Box<dyn FnMut(leptos::web_sys::CustomEvent)>);
            html_el.add_event_listener_with_callback("slider-change", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        });
    }

    view! {
        <SliderPrimitive
            node_ref=node_ref
            min={min}
            max={max}
            _step={step}
            value={clamped}
            orientation={orientation.as_str().to_string()}
            disabled={disabled}
            class={class}
            id={id.unwrap_or_default()}
        >
            <SliderTrackPrimitive>
                <SliderRangePrimitive />
                <SliderThumbPrimitive />
            </SliderTrackPrimitive>
        </SliderPrimitive>
    }
}

#[component]
pub fn SliderPreview() -> impl IntoView {
    view! { <Slider min=0.0 max=100.0 value=50.0 /> }
}
