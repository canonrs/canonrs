//! Toggle Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::integration::aria;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::interactive;

pub fn init(root: Element) {
    if root.has_attribute("data-rs-disabled") { return; }
    interactive::init(&root);
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, &root, "change", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let is_checked = e.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                .map(|i| i.checked()).unwrap_or(false);
            if is_checked {
                state::remove_state(&root_c, "off"); state::add_state(&root_c, "on"); aria::set_pressed(&root_c, true);
            } else {
                state::remove_state(&root_c, "on"); state::add_state(&root_c, "off"); aria::set_pressed(&root_c, false);
            }
        }
    });
}
