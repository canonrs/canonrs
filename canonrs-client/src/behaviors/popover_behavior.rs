#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, Element};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
fn mount_positioned_portal(
    doc: &web_sys::Document,
    portal_id: &str,
    content: &web_sys::Element,
    x: f64,
    y: f64,
    transform: &str,
) {
    unmount_positioned_portal(doc, portal_id);
    let body = match doc.body() { Some(b) => b, None => return };
    let portal = match doc.create_element("div").ok() { Some(p) => p, None => return };
    portal.set_attribute("data-rs-overlay-portal", portal_id).ok();
    let style = format!(
        "position:fixed;left:{}px;top:{}px;transform:{};z-index:9999;pointer-events:auto;",
        x, y, transform
    );
    portal.set_attribute("style", &style).ok();
    if let Ok(clone) = content.clone_node_with_deep(true) {
        if let Ok(el) = clone.dyn_into::<web_sys::Element>() {
            el.set_attribute("data-rs-state", "open").ok();
            portal.append_child(&el).ok();
        }
    }
    body.append_child(&portal).ok();
}

#[cfg(feature = "hydrate")]
fn unmount_positioned_portal(doc: &web_sys::Document, portal_id: &str) {
    if let Ok(Some(el)) = doc.query_selector(&format!("[data-rs-overlay-portal='{}']", portal_id)) {
        el.remove();
    }
}

#[cfg(feature = "hydrate")]
fn get_pid(root: &web_sys::Element, prefix: &str) -> String {
    let key = format!("data-rs-{}-pid", prefix);
    root.get_attribute(&key).unwrap_or_else(|| {
        let id = format!("{}-{}", prefix, (js_sys::Math::random() * 1e9) as u64);
        root.set_attribute(&key, &id).ok();
        id
    })
}

#[cfg(feature = "hydrate")]
fn resolve_position(rect: &web_sys::DomRect, side: &str) -> (f64, f64, &'static str) {
    let gap = 8.0;
    match side {
        "top"   => (rect.left(), rect.top() - gap, "translateY(-100%)"),
        "right" => (rect.right() + gap, rect.top(), "none"),
        "left"  => (rect.left() - gap, rect.top(), "translateX(-100%)"),
        _       => (rect.left(), rect.bottom() + gap, "none"),
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-popover", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-popover-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-popover-attached", "1").ok();
        let open_signal = state.open;
        let pid = get_pid(root, "popover");

        let trigger = match root.query_selector("[data-rs-popover-trigger]").ok().flatten() {
            Some(t) => t,
            None => return Ok(()),
        };

        {
            let root_clone = root.clone();
            let trigger_clone = trigger.clone();
            let pid_clone = pid.clone();
            let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
                e.stop_propagation();
                let is_open = root_clone.get_attribute("data-rs-state").as_deref() == Some("open");
                let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
                if is_open {
                    open_signal.set(false);
                    root_clone.set_attribute("data-rs-state", "closed").ok();
                    trigger_clone.set_attribute("aria-expanded", "false").ok();
                    unmount_positioned_portal(&doc, &pid_clone);
                } else {
                    // fechar outros popovers abertos
                    if let Ok(open_popovers) = doc.query_selector_all("[data-rs-popover][data-rs-state='open']") {
                        let len = open_popovers.length();
                        for i in 0..len {
                            if let Some(other) = open_popovers.item(i) {
                                if let Ok(other_el) = other.dyn_into::<Element>() {
                                    if other_el != root_clone {
                                        other_el.set_attribute("data-rs-state", "closed").ok();
                                        if let Some(other_pid) = other_el.get_attribute("data-rs-popover-pid") {
                                            unmount_positioned_portal(&doc, &other_pid);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    open_signal.set(true);
                    root_clone.set_attribute("data-rs-state", "open").ok();
                    trigger_clone.set_attribute("aria-expanded", "true").ok();
                    if let Some(content) = root_clone.query_selector("[data-rs-popover-content]").ok().flatten() {
                        if let Some(t) = trigger_clone.dyn_ref::<HtmlElement>() {
                            let rect = t.get_bounding_client_rect();
                            let side = content.get_attribute("data-rs-side").unwrap_or_else(|| "bottom".to_string());
                            let (x, y, transform) = resolve_position(&rect, &side);
                            mount_positioned_portal(&doc, &pid_clone, &content, x, y, transform);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            trigger.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
            cb.forget();
        }

        let root_outside = root.clone();
        let pid_outside = pid.clone();
        let cb_outside = Closure::wrap(Box::new(move |e: MouseEvent| {
            if root_outside.get_attribute("data-rs-state").as_deref() != Some("open") { return; }
            let target = match e.target().and_then(|t| t.dyn_into::<Element>().ok()) { Some(t) => t, None => return };
            let in_any_trigger = target.closest("[data-rs-popover-trigger]").ok().flatten().is_some();
            if in_any_trigger { return; }
            let in_popover = target.closest("[data-rs-popover]").ok().flatten().is_some();
            let in_portal = target.closest(&format!("[data-rs-overlay-portal='{}']", pid_outside)).ok().flatten().is_some();
            if !in_popover && !in_portal {
                open_signal.set(false);
                root_outside.set_attribute("data-rs-state", "closed").ok();
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    unmount_positioned_portal(&doc, &pid_outside);
                }
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window().unwrap().document().unwrap().add_event_listener_with_callback("click", cb_outside.as_ref().unchecked_ref()).ok();
        cb_outside.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
