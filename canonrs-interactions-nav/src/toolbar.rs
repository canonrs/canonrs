//! Toolbar Interaction Engine
//! Core: dom/{query} + behavior/keyboard::init_nav

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::{query};
use canonrs_interactions_core::behavior::keyboard::{init_nav, NavConfig, Orientation, ElementType};

fn dispatch_action(root: &Element, value: &str, pressed: bool) {
    use web_sys::CustomEventInit;
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &"value".into(), &value.into()).ok();
    js_sys::Reflect::set(&detail, &"pressed".into(), &pressed.into()).ok();
    let init = CustomEventInit::new();
    init.set_bubbles(true);
    init.set_detail(&detail);
    if let Ok(evt) = web_sys::CustomEvent::new_with_event_init_dict("canon:toolbar:action", &init) {
        let _ = root.dispatch_event(&evt);
    }
}

pub fn init(root: Element) {

    let is_vertical = root.get_attribute("data-rs-variant").as_deref() == Some("vertical");

    // keyboard — roving tabindex via core::behavior::keyboard::init_nav
    init_nav(
        &root,
        "[data-rs-toolbar-item]:not([disabled])",
        NavConfig {
            orientation:  if is_vertical { Orientation::Vertical } else { Orientation::Horizontal },
            element_type: ElementType::Button,
            focus_state:  "focused",
            wrap:         true,
        },
        None,
        None,
    );

    // click — toggle aria-pressed + dispatch CustomEvent
    {
        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(item) = target.closest("[data-rs-toolbar-item]").ok().flatten() else { return };
            if item.has_attribute("disabled") { return; }
            let pressed = item.get_attribute("aria-pressed").as_deref() == Some("true");
            let next = !pressed;
            let _ = item.set_attribute("aria-pressed", if next { "true" } else { "false" });
            let value = item.get_attribute("data-rs-value").unwrap_or_default();
            dispatch_action(&root_cb, &value, next);
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    }

    // init tabindex — primeiro item recebe 0, resto -1
    let items = query::all(&root, "[data-rs-toolbar-item]:not([disabled])");
    for (i, item) in items.iter().enumerate() {
        let _ = item.set_attribute("tabindex", if i == 0 { "0" } else { "-1" });
    }
}
