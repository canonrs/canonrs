//! Menu Init — single item selection + keyboard navigation + hover

use web_sys::Element;
use crate::runtime::{lifecycle, selection, keyboard, interactive, query};

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }
    selection::init_single(&root, "[data-rs-menu-item]");
    keyboard::init_navigation(&root, "[data-rs-menu-item]", None);

    // ao clicar num item → marcar como active para keyboard saber de onde continuar
    // e remover focus ring dos outros
    for item in query::all(&root, "[data-rs-menu-item]") {
        interactive::init(&item);
        let item_cb = item.clone();
        let root_cb = root.clone();
        let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |_: web_sys::MouseEvent| {
            for el in query::all(&root_cb, "[data-rs-menu-item]") {
                crate::runtime::state::remove_state(&el, "active");
                crate::runtime::state::remove_state(&el, "focus");
            }
            crate::runtime::state::add_state(&item_cb, "active");
        });
        use wasm_bindgen::JsCast;
        let _ = item.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
