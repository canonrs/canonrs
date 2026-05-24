//! Observer — governed MutationObserver and IntersectionObserver kernel
//!
//! Replaces direct observer creation with uid-namespaced, owned observers.
//! All observers registered here are tracked in the ownership tree.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// Register a MutationObserver owned by uid
/// Observes childList + subtree by default
pub fn mutation(uid: &str, target: &Element, cb: impl Fn(js_sys::Array) + 'static) {
    let uid = uid.to_string();
    let closure = Closure::wrap(Box::new(move |mutations: js_sys::Array, _: web_sys::MutationObserver| {
        cb(mutations);
    }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    let init = web_sys::MutationObserverInit::new();
    init.set_child_list(true);
    init.set_subtree(true);
    init.set_attributes(false);

    if let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) {
        observer.observe_with_options(target, &init).ok();
        // Store observer on element to prevent GC
        let key = format!("__canon_mo_{}", uid);
        let _ = js_sys::Reflect::set(target, &key.into(), &observer.into());
    }
    closure.forget(); // observer lives as long as element
}

/// Register a MutationObserver with custom options
pub fn mutation_opts(
    uid: &str,
    target: &Element,
    init: &web_sys::MutationObserverInit,
    cb: impl Fn(js_sys::Array) + 'static,
) {
    let uid = uid.to_string();
    let closure = Closure::wrap(Box::new(move |mutations: js_sys::Array, _: web_sys::MutationObserver| {
        cb(mutations);
    }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    if let Ok(observer) = web_sys::MutationObserver::new(closure.as_ref().unchecked_ref()) {
        observer.observe_with_options(target, init).ok();
        let key = format!("__canon_mo_{}", uid);
        let _ = js_sys::Reflect::set(target, &key.into(), &observer.into());
    }
    closure.forget();
}

/// Register an IntersectionObserver owned by uid
pub fn intersect(
    uid: &str,
    targets: &[Element],
    threshold: f64,
    cb: impl Fn(js_sys::Array) + 'static,
) {
    let closure = Closure::wrap(Box::new(move |entries: js_sys::Array, _: web_sys::IntersectionObserver| {
        cb(entries);
    }) as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    let opts = web_sys::IntersectionObserverInit::new();
    let thresholds = js_sys::Array::new();
    thresholds.push(&wasm_bindgen::JsValue::from_f64(threshold));
    opts.set_threshold(&thresholds);

    if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        closure.as_ref().unchecked_ref(), &opts
    ) {
        for target in targets {
            observer.observe(target);
        }
        // Store on first target if available
        if let Some(first) = targets.first() {
            let key = format!("__canon_io_{}", uid);
            let _ = js_sys::Reflect::set(first, &key.into(), &observer.into());
        }
    }
    closure.forget();
}
