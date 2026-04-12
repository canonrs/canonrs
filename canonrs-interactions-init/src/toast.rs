//! Toast Init — auto-dismiss + close button

use web_sys::Element;
use crate::runtime::{lifecycle, dismiss};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    let duration_ms = root.get_attribute("data-rs-duration")
        .and_then(|d| d.parse::<i32>().ok())
        .unwrap_or(5000);
    dismiss::init_with_timer(&root, "[data-rs-toast-close]", duration_ms);
}
