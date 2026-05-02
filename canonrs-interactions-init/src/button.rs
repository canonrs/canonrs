//! Button Init — hover/focus/active/disabled states

use web_sys::Element;
use crate::runtime::{lifecycle, interactive, state};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // projeta data-rs-disabled para data-rs-state
    let disabled = root.get_attribute("data-rs-disabled")
        .map(|v| v == "true" || v == "disabled")
        .unwrap_or(false);
    if disabled {
        state::add_state(&root, "disabled");
    }

    interactive::init(&root);
}
