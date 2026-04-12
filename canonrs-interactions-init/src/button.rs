//! Button Init — hover/focus/active states

use web_sys::Element;
use crate::runtime::{lifecycle, interactive};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    interactive::init(&root);
}
