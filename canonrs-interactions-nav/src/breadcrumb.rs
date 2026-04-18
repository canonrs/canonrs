//! Breadcrumb Interaction — hover + active state sync

use crate::runtime::{lifecycle, state, query};
use web_sys::Element;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // sync active — primitive já seta data-rs-state via activity_attrs
    for el in query::all(&root, "[data-rs-breadcrumb-link]") {
        if state::has(&el, "active") {
            let _ = el.set_attribute("aria-current", "page");
        } else {
            let _ = el.remove_attribute("aria-current");
        }
    }

}
