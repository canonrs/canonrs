//! Toast Init

use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::listeners;
use crate::runtime::dismiss;

pub fn init(root: Element) {
    let variant = root.get_attribute("data-rs-variant").unwrap_or_default();
    if variant == "error" {
        dismiss::init(&root, "[data-rs-toast-close]");
        return;
    }
    let default_duration = match variant.as_str() {
        "success" => 3000, "info" => 3000, "warning" => 6000, _ => 5000,
    };
    let duration_ms = root.get_attribute("data-rs-duration")
        .and_then(|d| d.parse::<i32>().ok()).unwrap_or(default_duration);
    dismiss::init_with_timer(&root, "[data-rs-toast-close]", duration_ms);

    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, &root, "mouseenter", {
        let r = root.clone();
        move |_: web_sys::Event| { state::add(  &r, canonrs_interactions_core::dom::state::State::Paused.as_str()); }
    });
    listeners::listen(&uid, &root, "mouseleave", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove(&r, canonrs_interactions_core::dom::state::State::Paused.as_str()); }
    });
}
