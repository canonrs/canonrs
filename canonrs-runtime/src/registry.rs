use std::collections::HashSet;
use std::cell::RefCell;
use web_sys::Element;

thread_local! {
    static INITED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub fn should_init(el: &Element) -> bool {
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        INITED.with(|set| {
            let mut s = set.borrow_mut();
            if s.contains(&uid) {
                false
            } else {
                s.insert(uid);
                true
            }
        })
    } else {
        true
    }
}

/// Remove uid do registry — chamar quando componente desmonta.
pub fn cleanup(el: &Element) {
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        INITED.with(|set| {
            set.borrow_mut().remove(&uid);
        });
    }
}

/// GC reativo — remove uids de elementos desconectados.
/// Chamado pelo observer de remocao, nao por polling.
pub fn gc_elements(removed: &[Element]) {
    INITED.with(|set| {
        let mut s = set.borrow_mut();
        for el in removed {
            if let Some(uid) = el.get_attribute("data-rs-uid") {
                s.remove(&uid);
            }
            // tambem limpa filhos removidos
            if let Ok(children) = el.query_selector_all("[data-rs-uid]") {
                for i in 0..children.length() {
                    if let Some(child) = children.item(i) {
                        use wasm_bindgen::JsCast;
                        if let Ok(child_el) = child.dyn_into::<Element>() {
                            if let Some(uid) = child_el.get_attribute("data-rs-uid") {
                                s.remove(&uid);
                            }
                        }
                    }
                }
            }
        }
    });
}

/// GC global — fallback para elementos desconectados nao capturados.
pub fn gc() {
    INITED.with(|set| {
        let mut s = set.borrow_mut();
        let doc = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };
        s.retain(|uid| {
            doc.query_selector(&format!("[data-rs-uid='{}']", uid))
                .ok()
                .flatten()
                .map(|el| el.is_connected())
                .unwrap_or(false)
        });
    });
}
