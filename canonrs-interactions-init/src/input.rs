//! Input Init — interactive states + data-rs-value sync

use web_sys::Element;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::runtime::interactive;

pub fn init(root: Element) {
    let _ = root.set_attribute("data-rs-initialized", "true");
    interactive::init(&root);

    // MutationObserver — sincroniza data-rs-value com node.value
    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(js_sys::Array)>::new(move |mutations: js_sys::Array| {
        for m in mutations.iter() {
            let m: web_sys::MutationRecord = m.dyn_into().unwrap();
            if m.attribute_name().as_deref() == Some("data-rs-value") {
                if let Ok(input) = root_cb.clone().dyn_into::<web_sys::HtmlInputElement>() {
                    let val = root_cb.get_attribute("data-rs-value").unwrap_or_default();
                                    input.set_value(&val);
                }
            }
        }
    });

    let opts = web_sys::MutationObserverInit::new();
    opts.set_attributes(true);
    let filter = js_sys::Array::of1(&"data-rs-value".into());
    opts.set_attribute_filter(&filter);

    if let Ok(observer) = web_sys::MutationObserver::new(cb.as_ref().unchecked_ref()) {
        let _ = observer.observe_with_options(&root, &opts);
        std::mem::forget(observer);
    }
    cb.forget();

    // escuta rs:input:reset para limpar value programaticamente
    let root_cb2 = root.clone();
    let reset_cb = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
        if let Ok(input) = root_cb2.clone().dyn_into::<web_sys::HtmlInputElement>() {
            input.set_value("");
        }
    });
    let _ = root.add_event_listener_with_callback("rs:input:reset", reset_cb.as_ref().unchecked_ref());
    reset_cb.forget();
}
