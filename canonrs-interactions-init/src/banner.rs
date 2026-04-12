//! Banner Init — dismiss via [data-rs-banner-close]

use web_sys::Element;
use crate::runtime::{lifecycle, dismiss};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    dismiss::init(&root, "[data-rs-banner-close]");
}
