//! Breadcrumb Interaction Engine
use canonrs_interactions_core::dom::{lifecycle, state, query};
use web_sys::Element;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    for el in query::all(&root, "[data-rs-breadcrumb-link]") {
        if state::has(&el, "active") {
            let _ = el.set_attribute("aria-current", "page");
        } else {
            let _ = el.remove_attribute("aria-current");
        }
    }
}
