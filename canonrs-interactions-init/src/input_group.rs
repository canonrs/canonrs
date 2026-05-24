//! InputGroup Init

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::query;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::focus;

pub fn init(root: Element) {
    focus::init_within(&root);
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, &root, "click", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            if target.closest("[data-rs-input-group-addon]").ok().flatten().is_some() {
                if let Some(input) = query::first(&root_c, "input, textarea") {
                    let _ = input.dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                }
            }
        }
    });
}
