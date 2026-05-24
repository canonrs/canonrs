//! Animate Init — ativa animacao via IntersectionObserver

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::observer;

pub fn init(root: Element) {
    let animation = root.get_attribute("data-rs-animation").unwrap_or_default();
    if animation == "none" || animation.is_empty() { return; }

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    let root_c = root.clone();

    observer::intersect(&uid, &[root.clone()], 0.1, move |entries: js_sys::Array| {
        for entry in entries.iter() {
            let entry = entry.unchecked_into::<web_sys::IntersectionObserverEntry>();
            if entry.is_intersecting() {
                let el = root_c.clone().unchecked_into::<web_sys::HtmlElement>();
                let style = el.style();
                let _ = style.set_property("animation", "none");
                let _ = root_c.clone().unchecked_into::<web_sys::HtmlElement>().offset_width();
                let _ = style.remove_property("animation");
                state::remove_state(&root_c, "inactive");
                state::add_state(&root_c, "active");
            } else {
                state::remove_state(&root_c, "active");
                state::add_state(&root_c, "inactive");
            }
        }
    });
}
