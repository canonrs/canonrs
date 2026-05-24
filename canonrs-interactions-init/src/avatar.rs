//! Avatar Init — image load/error fallback

use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::listeners;

pub fn init(root: Element) {
    let img = query::first(&root, "[data-rs-avatar-image]");
    if img.is_none() {
        state::add_state(&root, "fallback-open");
        state::remove_state(&root, "fallback-closed");
        return;
    }
    let img = img.unwrap();
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, &img, "error", {
        let root_c = root.clone();
        move |_: web_sys::Event| {
            state::remove_state(&root_c, "loading");
            state::add_state(&root_c, "error");
            state::add_state(&root_c, "image-closed");
            state::remove_state(&root_c, "image-open");
            state::add_state(&root_c, "fallback-open");
            state::remove_state(&root_c, "fallback-closed");
        }
    });

    listeners::listen(&uid, &img, "load", {
        let root_c = root.clone();
        move |_: web_sys::Event| { state::remove_state(&root_c, "loading"); }
    });
}
