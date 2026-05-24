#![deny(warnings)]
pub mod runtime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_all() {
    runtime::scan_and_init();
}

#[wasm_bindgen]
pub fn gc() {
    runtime::registry::gc();
}

#[wasm_bindgen]
pub fn init_subtree(el: web_sys::Element) {
    runtime::init_element(&el);
}

/// Plugin registration — externos registram handlers sem depender do core
/// rs-canvas-runtime chama: canonrs_interactions::register_interaction("canvas", fn)
/// Runtime instrumentation — lifecycle state of a component by uid
#[wasm_bindgen]
pub fn runtime_lifecycle_state(uid: &str) -> String {
    canonrs_interactions_core::runtime::lifecycle::state(uid)
        .map(|s| s.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Runtime instrumentation — ownership tree summary (listeners, timers, observers)
#[wasm_bindgen]
pub fn runtime_ownership_summary(uid: &str) -> String {
    match canonrs_interactions_core::runtime::ownership::summary(uid) {
        Some((l, t, o)) => format!("listeners:{} timers:{} observers:{}", l, t, o),
        None => "not_registered".to_string(),
    }
}

/// Runtime instrumentation — total resources across all components
#[wasm_bindgen]
pub fn runtime_total_resources() -> String {
    let (l, t, o) = canonrs_interactions_core::runtime::ownership::total_resources();
    format!("listeners:{} timers:{} observers:{}", l, t, o)
}

/// Runtime instrumentation — total active listeners
#[wasm_bindgen]
pub fn runtime_active_listeners() -> usize {
    canonrs_interactions_core::runtime::listeners::active_count()
}

/// Runtime instrumentation — active namespace uids
#[wasm_bindgen]
pub fn runtime_namespaces() -> js_sys::Array {
    let arr = js_sys::Array::new();
    let nss: Vec<String> = canonrs_interactions_core::runtime::listeners::namespaces();
    for ns in nss {
        arr.push(&wasm_bindgen::JsValue::from_str(&ns));
    }
    arr
}

/// Runtime instrumentation — orphan listeners (detached elements)
#[wasm_bindgen]
pub fn runtime_orphan_listeners() -> usize {
    canonrs_interactions_core::runtime::listeners::orphan_count()
}

/// Runtime instrumentation — initialized component count
#[wasm_bindgen]
pub fn runtime_initialized_count() -> usize {
    crate::runtime::registry::initialized_count()
}

pub fn register_interaction(group: &str, handler: fn(web_sys::Element)) {
    runtime::dispatcher::register_external(group.to_string(), handler);
}
