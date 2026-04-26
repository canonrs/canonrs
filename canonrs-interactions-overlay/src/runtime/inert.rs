//! Inert — isolamento de background para overlays modais
//! Usa data-rs-inert-{uid} para marcar elementos inertizados
//! Garante que o portal nunca receba inert

use wasm_bindgen::JsCast;
use web_sys::Element;

fn doc() -> Option<web_sys::Document> {
    web_sys::window().and_then(|w| w.document())
}

/// Aplica ou remove inert em todos os filhos do body exceto o portal do overlay
pub fn set_inert_background(active: bool, uid: &str, portal_attr: &str) {
    let Some(doc) = doc() else { return };
    let Some(body) = doc.body() else { return };

    // encontra o portal pelo uid
    let portal = doc.query_selector(&format!(
        "[{}][data-rs-owner='{}']", portal_attr, uid
    )).ok().flatten();

    let nodes = body.child_nodes();
    for i in 0..nodes.length() {
        let Some(node) = nodes.item(i) else { continue };
        let Some(child) = node.dyn_into::<Element>().ok() else { continue };

        // exclusão precisa: portal direto ou nó que contém o portal
        if let Some(ref p) = portal {
            if &child == p { continue; }
            if child.contains(Some(p)) { continue; }
        }
        // exclusão por atributo (fallback)
        if child.has_attribute(portal_attr) { continue; }
        if child.query_selector(&format!("[{}]", portal_attr)).ok().flatten().is_some() { continue; }

        let marker = format!("data-rs-inert-{}", uid);

        if active {
            if child.has_attribute(&marker) { continue; }
            let _ = child.set_attribute(&marker, "");
            let _ = child.set_attribute("inert", "");
        } else {
            if child.has_attribute(&marker) {
                let _ = child.remove_attribute(&marker);
                let _ = child.remove_attribute("inert");
            }
        }
    }
}
