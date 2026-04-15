//! InputGroup Init — focus-within state

use web_sys::Element;
use crate::runtime::{lifecycle, focus};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    focus::init_within(&root);

    // click em addon → foca o input
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use crate::runtime::query;
    let root_cb = root.clone();
    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        if target.closest("[data-rs-input-group-addon]").ok().flatten().is_some() {
            if let Some(input) = query::first(&root_cb, "input, textarea") {
                let _ = input.dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
            }
        }
    });
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
