use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserver, MutationObserverInit};

pub fn observe() {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let cb = Closure::wrap(Box::new(move |mutations: js_sys::Array, _: MutationObserver| {
        // coleta todos os nodes unicos de todas as mutations em burst
        let mut candidates: Vec<web_sys::Element> = vec![];
        let mut seen_uids: std::collections::HashSet<String> = std::collections::HashSet::new();

        let mut removed: Vec<web_sys::Element> = vec![];

        for m in mutations.iter() {
            let record: web_sys::MutationRecord = m.unchecked_into();

            // capturar nodes adicionados
            let added = record.added_nodes();
            for i in 0..added.length() {
                if let Some(node) = added.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        let key = el.get_attribute("data-rs-uid")
                            .unwrap_or_else(|| format!("{}", el.tag_name()));
                        if seen_uids.insert(key) {
                            let has_interaction = el.has_attribute("data-rs-interaction")
                                || el.query_selector("[data-rs-interaction]").ok().flatten().is_some();
                            if has_interaction {
                                candidates.push(el);
                            }
                        }
                    }
                }
            }

            // capturar nodes removidos para GC reativo
            let rem = record.removed_nodes();
            for i in 0..rem.length() {
                if let Some(node) = rem.item(i) {
                    if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                        if el.has_attribute("data-rs-uid")
                            || el.query_selector("[data-rs-uid]").ok().flatten().is_some() {
                            removed.push(el);
                        }
                    }
                }
            }
        }

        // GC reativo — limpa uids removidos imediatamente
        if !removed.is_empty() {
            crate::registry::gc_elements(&removed);
        }

        // processar batch de uma vez — sem reprocessamento em burst
        for el in candidates {
            crate::init_element(&el);
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
