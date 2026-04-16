//! Progress Init — sync aria-valuenow + indicator transform via data-rs-value

use web_sys::Element;
use crate::runtime::{lifecycle, query, observer};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    sync(&root);

    let root_val = root.clone();
    observer::observe_attr(&root, "data-rs-value", move |_| { sync(&root_val); });

    let root_state = root.clone();
    observer::observe_attr(&root, "data-rs-state", move |_| { sync(&root_state); });
}

fn sync(el: &Element) {
    let s = el.get_attribute("data-rs-state").unwrap_or_default();
    let is_indeterminate = s.contains("indeterminate");
    let is_loading = s.contains("loading");

    if is_indeterminate || is_loading {
        let _ = el.remove_attribute("aria-valuenow");
        let _ = el.set_attribute("aria-busy", "true");
        return;
    }

    let _ = el.remove_attribute("aria-busy");

    let value = el
        .get_attribute("data-rs-value")
        .unwrap_or_else(|| "0".to_string())
        .parse::<f64>()
        .unwrap_or(0.0)
        .clamp(0.0, 100.0);

    let _ = el.set_attribute("aria-valuenow", &format!("{:.0}", value));

    if let Some(bar) = query::first(el, "[data-rs-progress-indicator]") {
        let pct = 100.0 - value;
        let _ = bar.set_attribute("style", &format!("transform: translateX(-{:.0}%)", pct));
    }
}
