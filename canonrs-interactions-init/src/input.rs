//! Input Init

use web_sys::Element;
use canonrs_interactions_core::runtime::{listeners, observer};
use crate::runtime::interactive;

pub fn init(root: Element) {
    interactive::init(&root);
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    // MutationObserver — sync data-rs-value with input.value
    let opts = web_sys::MutationObserverInit::new();
    opts.set_attributes(true);
    let filter = js_sys::Array::of1(&"data-rs-value".into());
    opts.set_attribute_filter(&filter);
    observer::mutation_opts(&uid, &root, &opts, {
        let root_c = root.clone();
        move |_: js_sys::Array| {
            use wasm_bindgen::JsCast;
            if let Ok(input) = root_c.clone().dyn_into::<web_sys::HtmlInputElement>() {
                let val = root_c.get_attribute("data-rs-value").unwrap_or_default();
                input.set_value(&val);
            }
        }
    });

    let uid_reset = format!("{}:reset", uid);
    listeners::listen(&uid_reset, &root, "rs:input:reset", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            use wasm_bindgen::JsCast;
            if let Ok(input) = root_c.clone().dyn_into::<web_sys::HtmlInputElement>() {
                input.set_value("");
            }
        }
    });
}
