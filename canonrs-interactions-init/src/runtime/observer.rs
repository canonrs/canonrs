//! Observer — MutationObserver helper para reagir a mudancas de atributos

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, MutationObserver, MutationObserverInit};

pub fn observe_attr(root: &Element, attr: &'static str, cb: impl Fn(Element) + 'static) {
    let root_obs = root.clone();
    let observer_cb = Closure::<dyn Fn(js_sys::Array)>::new(move |mutations: js_sys::Array| {
        for i in 0..mutations.length() {
            if let Ok(r) = mutations.get(i).dyn_into::<web_sys::MutationRecord>() {
                if r.attribute_name().as_deref() == Some(attr) {
                    cb(root_obs.clone());
                }
            }
        }
    });

    if let Ok(observer) = MutationObserver::new(observer_cb.as_ref().unchecked_ref()) {
        let filter = js_sys::Array::of1(&JsValue::from_str(attr));
        let opts = MutationObserverInit::new();
        opts.set_attributes(true);
        opts.set_attribute_filter(&filter);
        let _ = observer.observe_with_options(root, &opts);
        // observer kept alive via observer_cb.forget()
    }
    observer_cb.forget();
}
