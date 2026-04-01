#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use crate::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{HtmlElement, MouseEvent, FocusEvent};
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
    let style = format!("position:fixed;left:{}px;top:{}px;transform:{};z-index:9999;pointer-events:auto;", x, y, transform);
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
pub fn register() {
    register_behavior("data-rs-hover-card", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-hover-card-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-hover-card-attached", "1").ok();
        let open_signal = state.open;
        let pid = get_pid(root, "hover-card");

        let trigger: HtmlElement = match root.query_selector("[data-rs-hover-card-trigger]")
            .ok().flatten().and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
            Some(el) => el,
            None => return Ok(()),
        };

        let root_show = root.clone();
        let trigger_show = trigger.clone();
        let pid_show = pid.clone();
        let cb_show = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(true);
            root_show.set_attribute("data-rs-state", "open").ok();
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            if let Ok(Some(content)) = root_show.query_selector("[data-rs-hover-card-content]") {
                let rect = trigger_show.get_bounding_client_rect();
                let x = rect.left() + rect.width() / 2.0;
                let y = rect.bottom() + 8.0;
                mount_positioned_portal(&doc, &pid_show, &content, x, y, "translateX(-50%)");
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("mouseenter", cb_show.as_ref().unchecked_ref()).ok();
        cb_show.forget();

        let root_hide = root.clone();
        let pid_hide = pid.clone();
        let cb_hide = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(false);
            root_hide.set_attribute("data-rs-state", "closed").ok();
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                unmount_positioned_portal(&doc, &pid_hide);
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("mouseleave", cb_hide.as_ref().unchecked_ref()).ok();
        cb_hide.forget();

        let root_focus = root.clone();
        let trigger_focus = trigger.clone();
        let pid_focus = pid.clone();
        let cb_focus = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(true);
            root_focus.set_attribute("data-rs-state", "open").ok();
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            if let Ok(Some(content)) = root_focus.query_selector("[data-rs-hover-card-content]") {
                let rect = trigger_focus.get_bounding_client_rect();
                let x = rect.left() + rect.width() / 2.0;
                let y = rect.bottom() + 8.0;
                mount_positioned_portal(&doc, &pid_focus, &content, x, y, "translateX(-50%)");
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("focus", cb_focus.as_ref().unchecked_ref()).ok();
        cb_focus.forget();

        let root_blur = root.clone();
        let pid_blur = pid.clone();
        let cb_blur = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(false);
            root_blur.set_attribute("data-rs-state", "closed").ok();
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                unmount_positioned_portal(&doc, &pid_blur);
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("blur", cb_blur.as_ref().unchecked_ref()).ok();
        cb_blur.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
