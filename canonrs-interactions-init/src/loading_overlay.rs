//! LoadingOverlay Init — aria-busy + state management

use web_sys::Element;
use crate::runtime::lifecycle;

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    // state inicial já vem do SSR via data-rs-state
    // sync aria-busy com state
    let state = root.get_attribute("data-rs-state").unwrap_or_default();
    let is_loading = state.split_whitespace().any(|s| s == "loading");
    let _ = root.set_attribute("aria-busy", if is_loading { "true" } else { "false" });
}
