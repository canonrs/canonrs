//! overlay_portal — utilitário compartilhado para overlays com portal no <body>
//! Garante que position:fixed funcione corretamente independente de stacking context

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

/// Move o overlay content para o <body> como portal
/// Retorna o portal wrapper criado
#[cfg(feature = "hydrate")]
pub fn mount_portal(
    doc: &web_sys::Document,
    content: &Element,
    portal_id: &str,
) -> Option<Element> {
    let body = doc.body()?;

    // remover portal existente se houver
    unmount_portal(doc, portal_id);

    let portal = doc.create_element("div").ok()?;
    portal.set_attribute("data-rs-overlay-portal", portal_id).ok();
    portal.set_attribute("style", "position:fixed;inset:0;z-index:9999;pointer-events:none;").ok();

    // mover content para portal
    if let Ok(clone) = content.clone_node_with_deep(true) {
        if let Ok(el) = clone.dyn_into::<Element>() {
            el.set_attribute("style", "pointer-events:auto;").ok();
            portal.append_child(&el).ok();
        }
    }

    body.append_child(&portal).ok();
    Some(portal)
}

/// Remove o portal do <body>
#[cfg(feature = "hydrate")]
pub fn unmount_portal(doc: &web_sys::Document, portal_id: &str) {
    let sel = format!("[data-rs-overlay-portal='{}']", portal_id);
    if let Ok(Some(existing)) = doc.query_selector(&sel) {
        existing.remove();
    }
}
