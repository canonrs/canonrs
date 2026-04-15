//! Menu Init — single item selection + keyboard navigation + hover

use web_sys::Element;
use crate::runtime::{lifecycle, selection, keyboard, interactive, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    selection::init_single(&root, "[data-rs-menu-item]");
    keyboard::init_navigation(&root, "[data-rs-menu-item]", None);
    for item in query::all(&root, "[data-rs-menu-item]") {
        interactive::init(&item);
    }
}
