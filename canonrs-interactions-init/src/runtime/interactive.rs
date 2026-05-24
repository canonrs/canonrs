//! Interactive — hover/focus/active states para button, icon_button

use web_sys::Element;
use canonrs_interactions_core::dom::state;
use canonrs_interactions_core::runtime::listeners;

fn is_disabled(el: &Element) -> bool {
    el.get_attribute("data-rs-state").map(|s| s.contains("disabled")).unwrap_or(false)
}

pub fn init(root: &Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();

    listeners::listen(&uid, root, "mouseenter", {
        let r = root.clone();
        move |_: web_sys::Event| { if !is_disabled(&r) { state::add_state(&r, "hover"); } }
    });

    listeners::listen(&uid, root, "mouseleave", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "hover"); state::remove_state(&r, "active"); }
    });

    listeners::listen(&uid, root, "pointerdown", {
        let r = root.clone();
        move |_: web_sys::Event| { if !is_disabled(&r) { state::add_state(&r, "active"); } }
    });

    listeners::listen(&uid, root, "pointerup", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "active"); }
    });

    listeners::listen(&uid, root, "pointercancel", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "active"); }
    });

    listeners::listen(&uid, root, "focus", {
        let r = root.clone();
        move |_: web_sys::Event| { if !is_disabled(&r) { state::add_state(&r, "focus"); } }
    });

    listeners::listen(&uid, root, "blur", {
        let r = root.clone();
        move |_: web_sys::Event| { state::remove_state(&r, "focus"); }
    });
}
