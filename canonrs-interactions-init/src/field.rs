//! Field Init — focus-within + aria-describedby + label linking

use web_sys::Element;
use crate::runtime::{lifecycle, focus, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    focus::init_within(&root);

    // conectar label → input via data-rs-uid
    if let (Some(input), Some(label)) = (
        query::first(&root, "input, textarea, select"),
        query::first(&root, "[data-rs-field-label]")
    ) {
        let uid = input.get_attribute("data-rs-uid").unwrap_or_default();
        if !uid.is_empty() {
            let _ = input.set_attribute("id", &uid);
            let _ = label.set_attribute("for", &uid);
        }
    }

    // conectar aria-describedby: description + error → input
    let mut describedby = Vec::new();
    if let Some(desc) = query::first(&root, "[data-rs-field-description]") {
        let id = format!("fd-{}", root.get_attribute("data-rs-uid").unwrap_or_default());
        let _ = desc.set_attribute("id", &id);
        describedby.push(id);
    }
    if let Some(err) = query::first(&root, "[data-rs-field-error]") {
        let id = format!("fe-{}", root.get_attribute("data-rs-uid").unwrap_or_default());
        let _ = err.set_attribute("id", &id);
        describedby.push(id);
    }
    if !describedby.is_empty() {
        if let Some(input) = query::first(&root, "input, textarea, select") {
            let _ = input.set_attribute("aria-describedby", &describedby.join(" "));
        }
    }
}
