//! TableRowSheetPreview Init
//! Intercepta click em rows com data-rs-action="open-sheet"
//! Injeta dados no Sheet global e abre

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::runtime::lifecycle;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    // Gera uid para o context
    use std::sync::atomic::{AtomicU32, Ordering};
    static CTR: AtomicU32 = AtomicU32::new(0);
    let ctr = CTR.fetch_add(1, Ordering::Relaxed);
    let uid = format!("tc-{}", ctr);
    let _ = root.set_attribute("data-rs-uid", &uid);
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[table-sheet] context initialized uid={}", uid)));


    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

        // Ignora clique em celulas copyable
        if closest_attr(&target, "data-rs-copyable").is_some() { return; }

        // Sobe ate encontrar a row com data-rs-action="open-sheet"
        let row = closest_action(&target, "open-sheet");
        let Some(row) = row else {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[table-sheet] no row with open-sheet found"));
            return;
        };
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("[table-sheet] row clicked label={} meta={}", row.get_attribute("data-rs-label").unwrap_or_default(), row.get_attribute("data-rs-meta").unwrap_or_default())));

        let label = row.get_attribute("data-rs-label").unwrap_or_default();
        let meta  = row.get_attribute("data-rs-meta").unwrap_or_default();

        // Encontra o Sheet global no mesmo contexto (data-rs-table-context)
        let context = closest_attr(&row, "data-rs-table-context");
        let Some(ctx) = context else { return };

        let sheet = ctx.query_selector("[data-rs-sheet]").ok().flatten();
        let Some(sheet) = sheet else { return };

        // Injeta dados no sheet content
        if let Ok(Some(title)) = sheet.query_selector("[data-rs-sheet-title]") {
            let _ = title.set_text_content(Some(&label));
        }
        if let Ok(Some(desc)) = sheet.query_selector("[data-rs-sheet-description]") {
            let _ = desc.set_text_content(Some(&meta));
        }

        // Abre o sheet via state — mesmo padrão do overlay engine
        let current = sheet.get_attribute("data-rs-state").unwrap_or_default();
        let next: String = current.split_whitespace()
            .filter(|s| *s != "closed")
            .chain(std::iter::once("open"))
            .collect::<Vec<_>>()
            .join(" ");
        let _ = sheet.set_attribute("data-rs-state", &next);
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = doc.body() {
                let _ = body.set_attribute("data-rs-scroll-lock", "true");
            }
        }
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}

fn closest_action(el: &Element, action: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(el) = current {
        if el.get_attribute("data-rs-action").as_deref() == Some(action) {
            return Some(el);
        }
        current = el.parent_element();
    }
    None
}

fn closest_attr(el: &Element, attr: &str) -> Option<Element> {
    let mut current = Some(el.clone());
    while let Some(e) = current {
        if e.has_attribute(attr) {
            return Some(e);
        }
        current = e.parent_element();
    }
    None
}
