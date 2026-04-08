//! use_select — bridge entre Select behavior (DOM) e Leptos signals

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
pub fn use_select(signal: RwSignal<String>) -> NodeRef<leptos::html::Div> {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el_for_closure = el.clone();
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let root = el_for_closure.as_ref() as &web_sys::Element;
                if let Some(v) = root.get_attribute("data-rs-value") {
                    signal.set(v);
                }
            }) as Box<dyn FnMut(web_sys::Event)>);
            let root = el.as_ref() as &web_sys::Element;
            root.add_event_listener_with_callback("rs-change", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
    });
    node_ref
}

#[cfg(not(feature = "hydrate"))]
pub fn use_select(_signal: leptos::prelude::RwSignal<String>) -> leptos::prelude::NodeRef<leptos::html::Div> {
    leptos::prelude::NodeRef::new()
}
