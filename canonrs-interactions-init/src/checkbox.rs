//! Checkbox Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::focus;

pub fn init(root: Element) {
    let Some(input) = query::first(&root, "[data-rs-checkbox-input]") else { return };
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &input, "change", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let is_checked = e.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|i| i.checked()).unwrap_or(false);
            if is_checked {
                state::remove_state(&root_c, "unchecked");
                state::add_state(&root_c, "checked");
            } else {
                state::remove_state(&root_c, "checked");
                state::add_state(&root_c, "unchecked");
            }
        }
    });
    focus::init_within(&root);
}
