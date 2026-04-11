//! TableRowSheetPreview Init
//! Intercepta click em rows com data-rs-action="open-sheet"
//! Injeta dados no Sheet global e abre

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use crate::shared::{is_initialized, mark_initialized, add_state, remove_state};
use crate::runtime::query;

pub fn init(root: Element) {
    if is_initialized(&root) { return; }
    mark_initialized(&root);
    use std::sync::atomic::{AtomicU32, Ordering};
    static CTR: AtomicU32 = AtomicU32::new(0);
    let ctr = CTR.fetch_add(1, Ordering::Relaxed);
    let uid = format!("tc-{}", ctr);
    let _ = root.set_attribute("data-rs-uid", &uid);

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("[table-sheet] click detected"));
        if query::has_ancestor_attr(&target, "data-rs-copyable") { return; }

        let Some(row) = query::closest_action(&target, "open-sheet") else { return };

        let label = row.get_attribute("data-rs-label").unwrap_or_default();
        let meta  = row.get_attribute("data-rs-meta").unwrap_or_default();

        let Some(ctx) = query::closest_attr(&row, "data-rs-table-context") else { return };
        let Some(sheet) = query::first(&ctx, "[data-rs-sheet]") else { return };

        if let Some(title) = query::first(&sheet, "[data-rs-sheet-title]") {
            let _ = title.set_text_content(Some(&label));
        }
        if let Some(desc) = query::first(&sheet, "[data-rs-sheet-description]") {
            let _ = desc.set_text_content(Some(&meta));
        }

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

        // Marca row como selected
        let rows = query::all(&ctx, "[data-rs-action='open-sheet']");
        for r in &rows { remove_state(r, "selected"); }
        add_state(&row, "selected");
    });

    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
