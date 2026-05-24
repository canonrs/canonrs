//! Dismiss — close button pattern para alert, banner, toast

use web_sys::Element;
use canonrs_interactions_core::dom::{state, query};
use canonrs_interactions_core::runtime::{listeners, timers};

pub fn init(root: &Element, close_selector: &str) {
    let Some(btn) = query::first(root, close_selector) else { return };
    let uid = format!("dismiss:{}", root.get_attribute("data-rs-uid").unwrap_or_default());
    let root_c = root.clone();
    listeners::listen(&uid, &btn, "click", move |_: web_sys::Event| {
        state::remove_state(&root_c, "open");
        state::add_state(&root_c, "closed");
    });
}

pub fn init_with_timer(root: &Element, close_selector: &str, duration_ms: i32) {
    init(root, close_selector);
    let root_c = root.clone();
    timers::timeout(duration_ms, move || {
        let current = root_c.get_attribute("data-rs-state").unwrap_or_default();
        if current.contains("paused") { return; }
        state::remove_state(&root_c, "open");
        state::add_state(&root_c, "closed");
    });
}
