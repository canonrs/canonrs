//! Button Init — hover/focus/active/disabled states

use web_sys::Element;
use canonrs_interactions_core::dom::{lifecycle, state};
use crate::runtime::{interactive};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // data-rs-disabled="" significa disabled (atributo presente = disabled)
    let disabled = root.has_attribute("data-rs-disabled");
    if disabled {
        state::add_state(&root, "disabled");
    }

    interactive::init(&root);
}
