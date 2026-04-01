#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, KeyboardEvent, Element};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
thread_local! {
    static OPEN_COUNT: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

#[cfg(feature = "hydrate")]
fn lock_scroll() {
    OPEN_COUNT.with(|c| {
        let n = c.get() + 1; c.set(n);
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
        let n = c.get().saturating_sub(1); c.set(n);
        if n == 0 {
            if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
                body.style().remove_property("overflow").ok();
            }
        }
    });
}

#[cfg(feature = "hydrate")]
fn get_focusable(container: &Element) -> Vec<Element> {
    let sel = "a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\"-1\"])";
    let mut out = vec![];
    if let Ok(list) = container.query_selector_all(sel) {
        for i in 0..list.length() {
            if let Some(n) = list.item(i) {
                if let Ok(el) = n.dyn_into::<Element>() { out.push(el); }
            }
        }
    }
    out
}

#[cfg(feature = "hydrate")]
fn get_portal_id(root: &Element, prefix: &str) -> String {
    root.get_attribute(&format!("data-rs-{}-portal-id", prefix)).unwrap_or_else(|| {
        let id = format!("{}-portal-{}", prefix, (js_sys::Math::random() * 1e9) as u64);
        root.set_attribute(&format!("data-rs-{}-portal-id", prefix), &id).ok();
        id
    })
}

#[cfg(feature = "hydrate")]
fn mount_portal(root: &Element, doc: &web_sys::Document, portal_id: &str, overlay_sel: &str, content_sel: &str) {
    unmount_portal(doc, portal_id);
    let body = match doc.body() { Some(b) => b, None => return };
    let portal = match doc.create_element("div").ok() { Some(p) => p, None => return };
    portal.set_attribute("data-rs-overlay-portal", portal_id).ok();
    if let Ok(Some(overlay)) = root.query_selector(overlay_sel) {
        if let Ok(clone) = overlay.clone_node_with_deep(true) { portal.append_child(&clone).ok(); }
    }
    if let Ok(Some(content)) = root.query_selector(content_sel) {
        if let Ok(clone) = content.clone_node_with_deep(true) { portal.append_child(&clone).ok(); }
    }
    body.append_child(&portal).ok();
}

#[cfg(feature = "hydrate")]
fn unmount_portal(doc: &web_sys::Document, portal_id: &str) {
    let _sel = format!("[data-rs-overlay-portal=\'{}\']", portal_id);
    if let Ok(Some(el)) = doc.query_selector(&format!("[data-rs-overlay-portal='{}']", portal_id)) {
        el.remove();
    }
}

#[cfg(feature = "hydrate")]
fn close_drawer(root: &Element, trigger: &Option<Element>, open_signal: leptos::prelude::RwSignal<bool>, portal_id: String) {
    root.set_attribute("data-rs-state", "closed").ok();
    open_signal.set(false);
    if let Some(t) = trigger { t.set_attribute("aria-expanded", "false").ok(); }
    unlock_scroll();
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        unmount_portal(&doc, &portal_id);
    }
    if let Some(t) = trigger {
        if let Ok(el) = t.clone().dyn_into::<HtmlElement>() { el.focus().ok(); }
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-drawer", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-drawer-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-drawer-attached", "1").ok();

        let open_signal = state.open;
        let trigger = root.query_selector("[data-rs-drawer-trigger]").ok().flatten();
        let portal_id = get_portal_id(root, "drawer");

        if let Some(ref t) = trigger {
            let root_clone = root.clone();
            let trigger_clone = trigger.clone();
            let portal_id_clone = portal_id.clone();
            let cb = Closure::wrap(Box::new(move |_: MouseEvent| {
                let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                if is_open {
                    close_drawer(&root_clone, &trigger_clone, open_signal, portal_id_clone.clone());
                } else {
                    root_clone.set_attribute("data-rs-state", "open").ok();
                    open_signal.set(true);
                    if let Some(ref t) = trigger_clone { t.set_attribute("aria-expanded", "true").ok(); }
                    lock_scroll();
                    let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
                    mount_portal(&root_clone, &doc, &portal_id_clone, "[data-rs-drawer-overlay]", "[data-rs-drawer-content]");
                    let pid = portal_id_clone.clone();
                    let root_del = root_clone.clone();
                    let trigger_del = trigger_clone.clone();
                    let cb_del = Closure::wrap(Box::new(move |e: MouseEvent| {
                        if root_del.get_attribute("data-rs-state").as_deref() != Some("open") { return; }
                        let target = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) { Some(t) => t, None => return };
                        let is_overlay = target.has_attribute("data-rs-drawer-overlay") || target.closest("[data-rs-drawer-overlay]").ok().flatten().is_some();
                        let is_close = target.has_attribute("data-rs-drawer-close") || target.closest("[data-rs-drawer-close]").ok().flatten().is_some();
                        if is_overlay || is_close {
                            close_drawer(&root_del, &trigger_del, open_signal, pid.clone());
                        }
                    }) as Box<dyn FnMut(_)>);
                    doc.add_event_listener_with_callback("click", cb_del.as_ref().unchecked_ref()).ok();
                    cb_del.forget();
                    let portal_sel = format!("[data-rs-overlay-portal='{}']", portal_id_clone);
                    if let Ok(Some(portal)) = doc.query_selector(&portal_sel) {
                        let focusable = get_focusable(&portal);
                        if let Some(first) = focusable.first() {
                            if let Ok(el) = first.clone().dyn_into::<HtmlElement>() {
                            let win = web_sys::window().unwrap();
                            let sx = win.scroll_x().unwrap_or(0.0);
                            let sy = win.scroll_y().unwrap_or(0.0);
                            el.focus().ok();
                            win.scroll_to_with_x_and_y(sx, sy);
                        }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            t.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        let root_esc = root.clone();
        let trigger_esc = trigger.clone();
        let portal_id_esc = portal_id.clone();
        let cb_esc = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" && root_esc.get_attribute("data-rs-state").as_deref() == Some("open") {
                close_drawer(&root_esc, &trigger_esc, open_signal, portal_id_esc.clone());
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().add_event_listener_with_callback("keydown", cb_esc.as_ref().unchecked_ref()).ok();
        cb_esc.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
