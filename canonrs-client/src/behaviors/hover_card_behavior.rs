#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_core::BehaviorResult;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::{MouseEvent, HtmlElement, FocusEvent};
#[cfg(feature = "hydrate")]
use leptos::prelude::Set;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-rs-hover-card", Box::new(|root: &web_sys::Element, state: &ComponentState| -> BehaviorResult<()> {

        if root.get_attribute("data-rs-hover-card-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-rs-hover-card-attached", "1").ok();

        let open_signal = state.open;

        let trigger: HtmlElement = match root.query_selector("[data-rs-trigger]")
            .ok().flatten()
            .and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
            Some(el) => el,
            None => return Ok(()),
        };

        let root_show = root.clone();
        let trigger_show = trigger.clone();
        let cb_show = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(true);
            root_show.set_attribute("data-rs-state", "open").ok();
            if let Ok(Some(content)) = root_show.query_selector("[data-rs-hover-card-content]") {
                if let Ok(c) = content.dyn_into::<HtmlElement>() {
                    let rect = trigger_show.get_bounding_client_rect();
                    let x = rect.left() + rect.width() / 2.0;
                    let y = rect.bottom() + 8.0;
                    c.style().set_property("left", &format!("{}px", x)).ok();
                    c.style().set_property("top", &format!("{}px", y)).ok();
                    c.style().set_property("transform", "translateX(-50%)").ok();
                }
            }
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("mouseenter", cb_show.as_ref().unchecked_ref()).ok();
        cb_show.forget();

        let root_hide = root.clone();
        let cb_hide = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(false);
            root_hide.set_attribute("data-rs-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("mouseleave", cb_hide.as_ref().unchecked_ref()).ok();
        cb_hide.forget();

        let root_focus = root.clone();
        let cb_focus = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(true);
            root_focus.set_attribute("data-rs-state", "open").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("focus", cb_focus.as_ref().unchecked_ref()).ok();
        cb_focus.forget();

        let root_blur = root.clone();
        let cb_blur = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(false);
            root_blur.set_attribute("data-rs-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("blur", cb_blur.as_ref().unchecked_ref()).ok();
        cb_blur.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
