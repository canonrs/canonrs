//! Menu Init — single item selection

use web_sys::Element;
use crate::runtime::{lifecycle, selection};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    selection::init_single(&root, "[data-rs-menu-item]");
}
