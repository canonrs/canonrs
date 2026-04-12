//! InputGroup Init — focus-within state

use web_sys::Element;
use crate::runtime::{lifecycle, focus};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    focus::init_within(&root);
}
