//! Button Init — hover/focus/active/disabled states

use web_sys::Element;
use canonrs_interactions_core::dom::{state};
use crate::runtime::{interactive};

pub fn init(root: Element) {

    // data-rs-disabled="" significa disabled (atributo presente = disabled)
    let disabled = root.has_attribute("data-rs-disabled");
    if disabled {
        state::add_state(&root, "disabled");
    }

    interactive::init(&root);
}
