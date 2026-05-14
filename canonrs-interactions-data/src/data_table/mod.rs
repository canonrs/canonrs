//! DataTable capability registry
//! Each module owns its lifecycle, DOM, state, and event model.

pub mod filter;
pub mod sort;
pub mod pagination;
pub mod density;
pub mod col_toggle;
pub mod selection;
pub mod actions;
pub mod col_resize;
pub mod col_reorder;
pub mod col_pin;
pub mod expand;

use web_sys::HtmlElement;
use canonrs_interactions_core::dom::lifecycle;
use wasm_bindgen::JsCast;

/// Capability registry — each cap::init is autonomous, receives &HtmlElement,
/// has no knowledge of other capabilities.
const CAPS: &[fn(&HtmlElement)] = &[
    filter::init,
    sort::init,
    pagination::init,
    density::init,
    col_toggle::init,
    selection::init,
    actions::init,
    col_resize::init,
    col_reorder::init,
    col_pin::init,
    expand::init,
];

pub fn init(root: web_sys::Element) {
    if !lifecycle::init_guard(&root) { return; }
    if let Ok(el) = root.dyn_into::<HtmlElement>() {
        for cap in CAPS { cap(&el); }
    }
}
