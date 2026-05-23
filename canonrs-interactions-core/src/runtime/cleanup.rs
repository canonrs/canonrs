//! Cleanup — centralized component lifecycle destroy
//!
//! Chamado quando um componente desmonta ou é re-renderizado.
//! Remove todos os recursos associados ao uid:
//!   - listeners (via runtime::listeners)
//!   - timer handles
//!   - init registry (via dom::lifecycle)
//!   - data-rs-initialized attr

use web_sys::Element;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::dom::lifecycle;

// Timer handles por uid — para cancelamento no cleanup
thread_local! {
    static TIMER_HANDLES: RefCell<HashMap<String, Vec<i32>>> = RefCell::new(HashMap::new());
}

/// Registra timer handle para cleanup posterior
pub fn track_timer(uid: &str, handle: i32) {
    if handle < 0 { return; }
    TIMER_HANDLES.with(|t| {
        t.borrow_mut()
            .entry(uid.to_string())
            .or_default()
            .push(handle);
    });
}

/// Cleanup completo de um componente por uid
/// Remove: listeners, timers, init state
pub fn cleanup_uid(uid: &str) {
    // 1. remove listeners do namespace
    super::listeners::cleanup(uid);

    // 2. cancela timers pendentes
    TIMER_HANDLES.with(|t| {
        if let Some(handles) = t.borrow_mut().remove(uid) {
            for h in handles {
                super::timers::cancel_timeout(h);
            }
        }
    });

    // 3. remove data-rs-initialized do elemento
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let sel = format!("[data-rs-uid=\"{}\"]", uid);
        if let Ok(Some(el)) = doc.query_selector(&sel) {
            lifecycle::reset(&el);
        }
    }
}

/// Cleanup de uma subtree inteira
/// Remove todos os componentes com data-rs-uid dentro do root
pub fn cleanup_subtree(root: &Element) {
    use wasm_bindgen::JsCast;

    // cleanup do proprio root
    if let Some(uid) = root.get_attribute("data-rs-uid") {
        cleanup_uid(&uid);
    }

    // cleanup de todos os descendentes com uid
    let Ok(nodes) = root.query_selector_all("[data-rs-uid]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(el)    = raw.dyn_into::<Element>() else { continue };
        if let Some(uid) = el.get_attribute("data-rs-uid") {
            cleanup_uid(&uid);
        }
    }
}

/// GC global — cleanup de todos os componentes desconectados
pub fn gc() {
    // gc de listeners
    super::listeners::gc();

    // gc de timers — remove handles de uids sem elemento no DOM
    TIMER_HANDLES.with(|t| {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        t.borrow_mut().retain(|uid, handles| {
            let sel = format!("[data-rs-uid=\"{}\"]", uid);
            let exists = doc.query_selector(&sel)
                .ok().flatten()
                .map(|el| el.is_connected())
                .unwrap_or(false);
            if !exists {
                for h in handles.iter() {
                    super::timers::cancel_timeout(*h);
                }
                return false;
            }
            true
        });
    });
}
