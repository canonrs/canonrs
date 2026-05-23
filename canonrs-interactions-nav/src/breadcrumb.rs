//! Breadcrumb Interaction Engine
//! Core: dom/{state, query}

use canonrs_interactions_core::dom::{state, query};
use web_sys::Element;

pub fn init(root: Element) {
    for el in query::all(&root, "[data-rs-breadcrumb-link]") {
        if state::has(&el, canonrs_interactions_core::dom::state::State::Active.as_str()) {
            let _ = el.set_attribute("aria-current", "page");
        } else {
            let _ = el.remove_attribute("aria-current");
        }
    }
}
