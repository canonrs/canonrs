#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::{BehaviorResult, BehaviorError};
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
    register_behavior("data-hover-card", Box::new(|id: &str, state: &ComponentState| -> BehaviorResult<()> {
        use leptos::leptos_dom::helpers::document;

        let Some(root) = document().get_element_by_id(id) else {
            return Err(BehaviorError::ElementNotFound { selector: id.into() });
        };
        if root.get_attribute("data-hover-card-attached").as_deref() == Some("1") { return Ok(()); }
        root.set_attribute("data-hover-card-attached", "1").ok();

        let open_signal = state.open;
        let root_id = id.to_string();

        let Some(trigger) = document().query_selector(&format!("[data-hover-card-trigger=\'{}\']", root_id)).ok().flatten() else {
            return Ok(());
        };

        let content_sel = format!("#{} [data-hover-card-content]", root_id);
        let root_clone = root.clone();
        let doc_show = document();
        let content_sel_show = content_sel.clone();
        let trigger_sel = format!("[data-hover-card-trigger=\'{}\']", root_id).replace("\'", "'");

        let cb_show = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(true);
            root_clone.set_attribute("data-state", "open").ok();
            if let Some(content) = doc_show.query_selector(&content_sel_show).ok().flatten() {
                if let (Ok(c), Some(t)) = (content.dyn_into::<HtmlElement>(), doc_show.query_selector(&trigger_sel).ok().flatten().and_then(|el| el.dyn_into::<HtmlElement>().ok())) {
                    let rect = t.get_bounding_client_rect();
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

        let root_clone2 = root.clone();
        let cb_hide = Closure::wrap(Box::new(move |_: MouseEvent| {
            open_signal.set(false);
            root_clone2.set_attribute("data-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("mouseleave", cb_hide.as_ref().unchecked_ref()).ok();
        cb_hide.forget();

        // Focus support
        let root_clone3 = root.clone();
        let cb_focus = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(true);
            root_clone3.set_attribute("data-state", "open").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("focus", cb_focus.as_ref().unchecked_ref()).ok();
        cb_focus.forget();

        let root_clone4 = root.clone();
        let cb_blur = Closure::wrap(Box::new(move |_: FocusEvent| {
            open_signal.set(false);
            root_clone4.set_attribute("data-state", "closed").ok();
        }) as Box<dyn FnMut(_)>);
        trigger.add_event_listener_with_callback("blur", cb_blur.as_ref().unchecked_ref()).ok();
        cb_blur.forget();

        Ok(())
    }));
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
