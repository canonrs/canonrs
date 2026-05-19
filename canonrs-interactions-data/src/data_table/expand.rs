//! Expand — row expand/collapse behavior
//! Core: dom/{state}
use canonrs_interactions_core::dom::lifecycle;
use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use canonrs_interactions_core::dom::state;

pub fn init(table: &HtmlElement) {
    if !lifecycle::init_guard(&table.clone().into()) { return; }
    let table_c = table.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let btn = if t.has_attribute("data-rs-datatable-expand-btn") {
            Some(t)
        } else {
            t.closest("[data-rs-datatable-expand-btn]").ok().flatten()
        };
        let Some(btn) = btn else { return };
        e.stop_propagation();

        let row_id = btn.get_attribute("data-rs-row-id").unwrap_or_default();
        // busca dentro da tabela — não no document global
        let root: web_sys::Element = table_c.clone().into();
        let selector = format!("[data-rs-datatable-expand-row][data-rs-row-id='{}']", row_id);
        let Ok(Some(expand_row)) = root.query_selector(&selector) else { return };

        if expand_row.has_attribute("hidden") {
            let _ = expand_row.remove_attribute("hidden");
            let _ = btn.set_attribute("aria-expanded", "true");
            state::expand(&btn);
        } else {
            let _ = expand_row.set_attribute("hidden", "");
            let _ = btn.set_attribute("aria-expanded", "false");
            state::collapse(&btn);
        }
    }));
    let _ = table.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
