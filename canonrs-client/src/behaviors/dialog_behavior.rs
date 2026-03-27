//! @canon-level: strict
//! Dialog Behavior — DOM as source of truth
//! aria-labelledby/describedby gerados automaticamente via DOM traversal

#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, KeyboardEvent, MouseEvent};

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
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
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
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                body.style().remove_property("overflow").ok();
            }
        }
    });
}

#[cfg(feature = "hydrate")]
fn get_focusable(container: &web_sys::Element) -> Vec<web_sys::Element> {
    let sel = concat!(
        "a[href],button:not([disabled]),input:not([disabled]),",
        "select:not([disabled]),textarea:not([disabled]),",
        "[tabindex]:not([tabindex='-1'])"
    );
    let mut out = vec![];
    if let Ok(list) = container.query_selector_all(sel) {
        for i in 0..list.length() {
            if let Some(n) = list.item(i) {
                if let Ok(el) = n.dyn_into::<web_sys::Element>() {
                    out.push(el);
                }
            }
        }
    }
    out
}

#[cfg(feature = "hydrate")]
fn wire_aria(root: &web_sys::Element) {
    if let Ok(Some(content)) = root.query_selector("[data-rs-dialog-content]") {
        if let Ok(Some(title)) = root.query_selector("[data-rs-dialog-title]") {
            let id = "rs-dialog-title";
            title.set_attribute("id", id).ok();
            content.set_attribute("aria-labelledby", id).ok();
        }
        if let Ok(Some(desc)) = root.query_selector("[data-rs-dialog-description]") {
            let id = "rs-dialog-desc";
            desc.set_attribute("id", id).ok();
            content.set_attribute("aria-describedby", id).ok();
        }
    }
}

#[cfg(feature = "hydrate")]
fn open_dialog(root: &web_sys::Element, trigger: &Option<web_sys::Element>) {
    root.set_attribute("data-rs-state", "open").ok();
    if let Some(t) = trigger {
        t.set_attribute("aria-expanded", "true").ok();
    }
    lock_scroll();
    let focusable = get_focusable(root);
    if let Some(first) = focusable.first() {
        if let Ok(el) = first.clone().dyn_into::<HtmlElement>() {
            el.focus().ok();
        }
    }
}

#[cfg(feature = "hydrate")]
fn close_dialog(root: &web_sys::Element, trigger: &Option<web_sys::Element>) {
    root.set_attribute("data-rs-state", "closed").ok();
    if let Some(t) = trigger {
        t.set_attribute("aria-expanded", "false").ok();
        if let Ok(el) = t.clone().dyn_into::<HtmlElement>() {
            el.focus().ok();
        }
    }
    unlock_scroll();
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior(
        "data-rs-dialog",
        Box::new(|root: &web_sys::Element, _state: &ComponentState| -> BehaviorResult<()> {
            if root.get_attribute("data-rs-dialog-attached").as_deref() == Some("1") {
                return Ok(());
            }
            root.set_attribute("data-rs-dialog-attached", "1").ok();

            wire_aria(root);

            let trigger = root.query_selector("[data-rs-dialog-trigger]").ok().flatten();

            // trigger click
            if let Some(ref t) = trigger {
                let root_c = root.clone();
                let trigger_c = trigger.clone();
                let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                    let is_open =
                        root_c.get_attribute("data-rs-state").as_deref() == Some("open");
                    if is_open {
                        close_dialog(&root_c, &trigger_c);
                    } else {
                        open_dialog(&root_c, &trigger_c);
                    }
                }) as Box<dyn FnMut(MouseEvent)>);
                t.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
                cb.forget();
            }

            // overlay click
            if let Ok(Some(overlay)) = root.query_selector("[data-rs-dialog-overlay]") {
                let root_c = root.clone();
                let trigger_c = trigger.clone();
                let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                    close_dialog(&root_c, &trigger_c);
                }) as Box<dyn FnMut(MouseEvent)>);
                overlay
                    .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
                    .ok();
                cb.forget();
            }

            // close buttons
            if let Ok(btns) = root.query_selector_all("[data-rs-dialog-close]") {
                for i in 0..btns.length() {
                    if let Some(node) = btns.item(i) {
                        if let Ok(btn) = node.dyn_into::<web_sys::Element>() {
                            let root_c = root.clone();
                            let trigger_c = trigger.clone();
                            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                                close_dialog(&root_c, &trigger_c);
                            }) as Box<dyn FnMut(MouseEvent)>);
                            btn.add_event_listener_with_callback(
                                "click",
                                cb.as_ref().unchecked_ref(),
                            )
                            .ok();
                            cb.forget();
                        }
                    }
                }
            }

            // ESC
            let root_esc = root.clone();
            let trigger_esc = trigger.clone();
            let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if e.key() == "Escape"
                    && root_esc.get_attribute("data-rs-state").as_deref() == Some("open")
                {
                    close_dialog(&root_esc, &trigger_esc);
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref())
                .ok();
            cb_esc.forget();

            // Tab trap
            let root_trap = root.clone();
            let cb_trap = Closure::wrap(Box::new(move |e: KeyboardEvent| {
                if root_trap.get_attribute("data-rs-state").as_deref() != Some("open")
                    || e.key() != "Tab"
                {
                    return;
                }
                let focusable = get_focusable(&root_trap);
                if focusable.is_empty() {
                    return;
                }
                let first = focusable.first().unwrap().clone();
                let last = focusable.last().unwrap().clone();
                let active = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.active_element());
                if e.shift_key() {
                    if active.as_ref() == Some(&first) {
                        e.prevent_default();
                        if let Ok(el) = last.dyn_into::<HtmlElement>() {
                            el.focus().ok();
                        }
                    }
                } else if active.as_ref() == Some(&last) {
                    e.prevent_default();
                    if let Ok(el) = first.dyn_into::<HtmlElement>() {
                        el.focus().ok();
                    }
                }
            }) as Box<dyn FnMut(KeyboardEvent)>);
            web_sys::window()
                .unwrap()
                .add_event_listener_with_callback("keydown", cb_trap.as_ref().unchecked_ref())
                .ok();
            cb_trap.forget();

            Ok(())
        }),
    );
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
