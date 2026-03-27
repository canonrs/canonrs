#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, KeyboardEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
thread_local! {
    static OPEN_COUNT: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

#[cfg(feature = "hydrate")]
fn lock_scroll() {
    OPEN_COUNT.with(|c| {
        let n = c.get() + 1;
        c.set(n);
        if n == 1 {
            if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                body.style().set_property("overflow", "hidden").ok();
            }
        }
    });
}

#[cfg(feature = "hydrate")]
fn unlock_scroll() {
    OPEN_COUNT.with(|c| {
        let n = c.get().saturating_sub(1);
        c.set(n);
        if n == 0 {
            if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                body.style().remove_property("overflow").ok();
            }
        }
    });
}

#[cfg(feature = "hydrate")]
fn get_focusable(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let sel = "a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let mut out = vec![];
    if let Ok(list) = container.query_selector_all(sel) {
        for i in 0..list.length() {
            if let Some(n) = list.item(i) {
                if let Ok(el) = n.dyn_into::<web_sys::Element>() { out.push(el); }
            }
        }
    }
    out
}

#[cfg(feature = "hydrate")]
fn close_sheet(root: &web_sys::Element, trigger: &Option<web_sys::Element>, open_signal: leptos::prelude::RwSignal<bool>) {
    root.set_attribute("data-rs-state", "closed").ok();
    open_signal.set(false);
    if let Some(t) = trigger {
        t.set_attribute("aria-expanded", "false").ok();
    }
    unlock_scroll();
    if let Some(t) = trigger {
        if let Ok(el) = t.clone().dyn_into::<HtmlElement>() { el.focus().ok(); }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-sheet", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-sheet-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-sheet-attached", "1").ok();

        let open_signal = state.open;
        let trigger = root.query_selector("[data-rs-sheet-trigger]").ok().flatten();

        if let Some(ref t) = trigger {
            let root_clone = root.clone();
            let trigger_clone = trigger.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                if is_open {
                    close_sheet(&root_clone, &trigger_clone, open_signal);
                } else {
                    root_clone.set_attribute("data-rs-state", "open").ok();
                    open_signal.set(true);
                    if let Some(ref t) = trigger_clone { t.set_attribute("aria-expanded", "true").ok(); }
                    lock_scroll();
                    let focusable = get_focusable(&root_clone);
                    if let Some(first) = focusable.first() {
                        if let Ok(el) = first.clone().dyn_into::<HtmlElement>() { el.focus().ok(); }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            t.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Ok(Some(overlay)) = root.query_selector("[data-rs-sheet-overlay]") {
            let root_clone = root.clone();
            let trigger_clone = trigger.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                close_sheet(&root_clone, &trigger_clone, open_signal);
            }) as Box<dyn FnMut(_)>);
            overlay.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        if let Ok(close_btns) = root.query_selector_all("[data-rs-sheet-close]") {
            for i in 0..close_btns.length() {
                if let Some(node) = close_btns.item(i) {
                    if let Ok(btn) = node.dyn_into::<web_sys::Element>() {
                        let root_clone = root.clone();
                        let trigger_clone = trigger.clone();
                        let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                            close_sheet(&root_clone, &trigger_clone, open_signal);
                        }) as Box<dyn FnMut(_)>);
                        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                        cb.forget();
                    }
                }
            }
        }

        let root_esc = root.clone();
        let trigger_esc = trigger.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" && root_esc.get_attribute("data-rs-state").as_deref() == Some("open") {
                close_sheet(&root_esc, &trigger_esc, open_signal);
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
