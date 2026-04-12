use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserver, MutationObserverInit};

pub fn observe() {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let cb = Closure::wrap(Box::new(move |mutations: js_sys::Array, _: MutationObserver| {
        for m in mutations.iter() {
            let record: web_sys::MutationRecord = m.unchecked_into();
            let nodes = record.added_nodes();
            for i in 0..nodes.length() {
                if let Some(node) = nodes.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        crate::runtime::init_element(&el);
                    }
                }
            }
        }
    }) as Box<dyn FnMut(js_sys::Array, MutationObserver)>);

    let observer = match MutationObserver::new(cb.as_ref().unchecked_ref()) {
        Ok(o) => o,
        Err(_) => return,
    };

    let opts = MutationObserverInit::new();
    opts.set_child_list(true);
    opts.set_subtree(true);
    let _ = observer.observe_with_options(&doc, &opts);
    cb.forget();
}
