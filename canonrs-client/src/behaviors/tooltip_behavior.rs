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
    let style = format!("position:fixed;left:{}px;top:{}px;transform:{};z-index:9999;pointer-events:none;", x, y, transform);
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
fn resolve_position(rect: &web_sys::DomRect, side: &str) -> (f64, f64, String) {
    let gap = 8.0;
    let cx = rect.left() + rect.width() / 2.0;
    let cy = rect.top() + rect.height() / 2.0;
    match side {
        "bottom" => (cx, rect.bottom() + gap, "translateX(-50%)".to_string()),
        "left"   => (rect.left() - gap, cy, "translateX(-100%) translateY(-50%)".to_string()),
        "right"  => (rect.right() + gap, cy, "translateY(-50%)".to_string()),
        _        => (cx, rect.top() - gap, "translateX(-50%) translateY(-100%)".to_string()),
    }
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-tooltip", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {
        if root.get_attribute("data-rs-tooltip-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-tooltip-attached", "1").ok();
        let open_signal = state.open;
        let pid = get_pid(root, "tooltip");

        let trigger: HtmlElement = match root.query_selector("[data-rs-tooltip-trigger]")
            .ok().flatten().and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
            Some(el) => el,
            None => return Ok(()),
        };

        let show = |root: &web_sys::Element, pid: &str, trigger: &HtmlElement| {
            let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
            if let Ok(Some(content)) = root.query_selector("[data-rs-tooltip-content]") {
                let side = content.get_attribute("data-rs-side").unwrap_or_else(|| "top".to_string());
                let rect = trigger.get_bounding_client_rect();
                let (x, y, transform) = resolve_position(&rect, &side);
                mount_positioned_portal(&doc, pid, &content, x, y, &transform);
            }
        };

        let root_show = root.clone();
        let trigger_show = trigger.clone();
        let pid_show = pid.clone();
        let cb_show = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(true);
            root_show.set_attribute("data-rs-state", "open").ok();
            show(&root_show, &pid_show, &trigger_show);
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
            show(&root_focus, &pid_focus, &trigger_focus);
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
