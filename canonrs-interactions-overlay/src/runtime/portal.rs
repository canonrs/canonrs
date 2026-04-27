//! Portal — helpers para overlays com portal (move para body, owner, nodes)

use wasm_bindgen::JsCast;
use web_sys::Element;

fn doc() -> Option<web_sys::Document> {
    web_sys::window().and_then(|w| w.document())
}

/// Busca o portal do overlay dentro do root ou no body pelo owner
pub fn portal_of(root: &Element, portal_attr: &str, uid: &str) -> Option<Element> {
    // 1. dentro do root
    root.query_selector(&format!("[{}]", portal_attr)).ok().flatten()
    // 2. no body com owner
    .or_else(|| doc()?.query_selector(&format!(
        "[{}][data-rs-owner=\'{}\']", portal_attr, uid
    )).ok().flatten())
    // 3. qualquer portal no body (fallback quando owner ainda nao foi propagado)
    .or_else(|| doc()?.query_selector(&format!("[{}]", portal_attr)).ok().flatten())
}

/// Propaga data-rs-owner para overlay e content dentro do portal
pub fn propagate_owner(portal: &Element, uid: &str, children_sel: &str) {
    let _ = portal.set_attribute("data-rs-owner", uid);
    if let Ok(nodes) = portal.query_selector_all(children_sel) {
        for i in 0..nodes.length() {
            if let Some(n) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
                let _ = n.set_attribute("data-rs-owner", uid);
            }
        }
    }
}

/// Move portal para body se ainda não estiver lá
pub fn move_to_body(portal: &Element, _uid: &str) {
    let Some(body) = doc().and_then(|d| d.body()) else { return };
    let in_body = portal.parent_element()
        .map(|p| p.tag_name() == "BODY")
        .unwrap_or(false);
    if !in_body {
        let _ = body.append_child(portal);

    }
}

/// Busca overlay e content pelo owner
pub fn portal_nodes(uid: &str, overlay_attr: &str, content_attr: &str) -> (Option<Element>, Option<Element>) {
    let Some(doc) = doc() else { return (None, None) };
    let overlay = doc.query_selector(&format!(
        "[{}][data-rs-owner='{}']", overlay_attr, uid
    )).ok().flatten();
    let content = doc.query_selector(&format!(
        "[{}][data-rs-owner='{}']", content_attr, uid
    )).ok().flatten();
    (overlay, content)
}
