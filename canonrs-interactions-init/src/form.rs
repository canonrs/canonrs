//! Form Init

use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::focus;

pub fn init(root: Element) {
    focus::init_within(&root);
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, &root, "submit", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            state::remove_state(&root_c, "idle");
            state::add_state(&root_c, "submitting");
        }
    });
    for field in query::all(&root, "[data-rs-form-field]") {
        if let (Some(input), Some(label)) = (
            query::first(&field, "input, textarea, select"),
            query::first(&field, "[data-rs-form-label]")
        ) {
            let uid_i = input.get_attribute("data-rs-uid").unwrap_or_default();
            if !uid_i.is_empty() {
                let _ = input.set_attribute("id", &uid_i);
                let _ = label.set_attribute("for", &uid_i);
            }
            if label.get_attribute("data-rs-required").as_deref() == Some("true") {
                let _ = input.set_attribute("required", "");
                let _ = input.set_attribute("aria-required", "true");
            }
        }
    }
}
