//! AlertDialog Interaction Engine
//! Alert dialogs are intentional — no Escape to close (destructive action)

use wasm_bindgen::prelude::*;
use web_sys::Element;
use crate::runtime::{lifecycle, state, query};

fn open(root: &Element)  { state::open(root);  state::set_scroll_lock(true); }
fn close(root: &Element) { state::close(root); state::set_scroll_lock(false); }

pub fn init(root: Element) {
    if !lifecycle::init_guard(&root) { return; }

    let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
        let Some(current) = query::safe_current(&e) else { return };
        let Some(target)  = query::safe_target(&e)  else { return };
        if query::closest(&target, "[data-rs-alert-dialog-trigger]") { open(&current); return; }
        if query::closest(&target, "[data-rs-alert-dialog-cancel]")  { close(&current); return; }
        if query::closest(&target, "[data-rs-alert-dialog-confirm]") { close(&current); }
    });
    let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    cb.forget();
}
