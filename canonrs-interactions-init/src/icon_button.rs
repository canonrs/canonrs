//! IconButton Init

use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::integration::aria;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::interactive;

pub fn init(root: Element) {
    interactive::init(&root);
    if root.has_attribute("aria-pressed") {
        let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
        listeners::listen(&uid, &root, "click", {
            let r = root.clone();
            move |_: web_sys::Event| {
                if r.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false) { return; }
                let is_on = r.get_attribute("data-rs-state").map(|s| s.contains("on")).unwrap_or(false);
                if is_on { state::remove_state(&r, "on"); aria::set_pressed(&r, false); }
                else     { state::add_state(&r, "on");    aria::set_pressed(&r, true); }
            }
        });
    }
}
