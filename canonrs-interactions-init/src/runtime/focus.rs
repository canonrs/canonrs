//! Focus — focus-within pattern para input_group, checkbox

use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::listeners;

pub fn init_within(root: &Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, root, "focusin", {
        let r = root.clone();
        move |_: web_sys::Event| { state::add_state(&r, "focus-within"); }
    });
    listeners::listen(&uid, root, "focusout", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "focus-within"); }
    });
}

pub fn init_focus(root: &Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    listeners::listen(&uid, root, "focusin", {
        let r = root.clone();
        move |_: web_sys::Event| { state::add_state(&r, "focus"); }
    });
    listeners::listen(&uid, root, "focusout", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "focus"); }
    });
}
