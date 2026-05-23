//! Bootstrap — centralized subtree init and replay
//!
//! Substitui init_all() e scan local de cada crate.
//! Um único ponto de entrada para inicializar qualquer subtree.
//!
//! Fluxo:
//!   init_subtree(root)
//!     → query [data-rs-interaction] dentro do root
//!     → lifecycle::init_guard (idempotente)
//!     → dispatch para fn registrada no DISPATCH_REGISTRY
//!     → replay-safe (pode ser chamado N vezes)

use wasm_bindgen::JsCast;
use web_sys::Element;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::dom::lifecycle;

// Registry de init fns por grupo — registradas por cada crate
thread_local! {
    static DISPATCH: RefCell<HashMap<String, fn(Element)>> = RefCell::new(HashMap::new());
}

/// Registra init fn para um grupo de interação
/// Chamado uma vez no bootstrap de cada crate
pub fn register(group: &str, init_fn: fn(Element)) {
    DISPATCH.with(|d| {
        d.borrow_mut().insert(group.to_string(), init_fn);
    });
}

/// Despacha para a fn registrada do grupo
fn dispatch(group: &str, el: Element) {
    DISPATCH.with(|d| {
        if let Some(f) = d.borrow().get(group) {
            f(el);
        }
    });
}

/// Inicializa todos os elementos [data-rs-interaction] dentro do root
/// Replay-safe: lifecycle::init_guard protege contra double-init
pub fn init_subtree(root: &Element) {
    // o proprio root pode ter data-rs-interaction
    if let Some(group) = root.get_attribute("data-rs-interaction") {
        if lifecycle::init_guard(root) {
            dispatch(&group, root.clone());
        }
    }

    // todos os descendentes
    let Ok(nodes) = root.query_selector_all("[data-rs-interaction]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(el)    = raw.dyn_into::<Element>() else { continue };
        if !el.is_connected() { continue }
        let Some(group) = el.get_attribute("data-rs-interaction") else { continue };
        if lifecycle::init_guard(&el) {
            dispatch(&group, el);
        }
    }
}

/// Inicializa o documento inteiro — usado no bootstrap inicial
pub fn init_document() {
    let Some(doc)  = web_sys::window().and_then(|w| w.document()) else { return };
    let Some(body) = doc.body() else { return };
    let body_el: Element = body.unchecked_into();
    init_subtree(&body_el);
}

/// Reinit forçado — ignora init_guard (para hydration replay explícito)
pub fn reinit_subtree(root: &Element) {
    // marca todos os elementos com data-rs-reinit
    let Ok(nodes) = root.query_selector_all("[data-rs-initialized]") else { return };
    for i in 0..nodes.length() {
        let Some(raw) = nodes.item(i) else { continue };
        let Ok(el)    = raw.dyn_into::<Element>() else { continue };
        let _ = el.set_attribute("data-rs-reinit", "");
    }
    // reinit com guard que respeita data-rs-reinit
    init_subtree(root);
}
