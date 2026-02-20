#[cfg(feature = "hydrate")]
use super::{register_behavior, ComponentState};
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::HtmlElement;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-list", Box::new(|id: &str, _state: &ComponentState| -> BehaviorResult<()> {
        let Some(list) = document().get_element_by_id(id) else { return Ok(()); };
        if list.get_attribute("data-list-attached").as_deref() == Some("1") { return Ok(()); }
        list.set_attribute("data-list-attached", "1").ok();

        let is_single = list.get_attribute("data-selection").as_deref() != Some("multiple");
        let list_id = id.to_string();

        let items = list.query_selector_all("[data-list-item-content][data-selectable]")
            .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query items".into() })?;

        for i in 0..items.length() {
            let node = match items.item(i) { Some(n) => n, None => continue };
            let item = match node.dyn_into::<HtmlElement>() { Ok(el) => el, Err(_) => continue };

            if item.has_attribute("data-disabled") { continue; }

            let list_id_clone = list_id.clone();
            let item_clone = item.clone();

            let on_select: std::rc::Rc<dyn Fn()> = std::rc::Rc::new(move || {
                let is_selected = item_clone.has_attribute("data-selected");

                // Single mode: deselect all siblings
                if is_single && !is_selected {
                    if let Some(list_el) = document().get_element_by_id(&list_id_clone) {
                        if let Ok(all) = list_el.query_selector_all("[data-list-item-content]") {
                            for j in 0..all.length() {
                                if let Some(n) = all.item(j) {
                                    if let Ok(el) = n.dyn_into::<HtmlElement>() {
                                        el.remove_attribute("data-selected").ok();
                                        el.set_attribute("aria-selected", "false").ok();
                                    }
                                }
                            }
                        }
                    }
                }

                if is_selected && !is_single {
                    item_clone.remove_attribute("data-selected").ok();
                    item_clone.set_attribute("aria-selected", "false").ok();
                    dispatch_select_event(&item_clone, false);
                } else if !is_selected {
                    item_clone.set_attribute("data-selected", "").ok();
                    item_clone.set_attribute("aria-selected", "true").ok();
                    dispatch_select_event(&item_clone, true);
                }
            });

            let on_select_rc = on_select;

            let on_select_click = on_select_rc.clone();
            let cb_click = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                on_select_click();
            }) as Box<dyn FnMut(_)>);
            item.add_event_listener_with_callback("click", cb_click.as_ref().unchecked_ref()).ok();
            cb_click.forget();

            let cb_key = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Enter" || e.key() == " " {
                    e.prevent_default();
                    on_select_rc();
                }
            }) as Box<dyn FnMut(_)>);
            item.add_event_listener_with_callback("keydown", cb_key.as_ref().unchecked_ref()).ok();
            cb_key.forget();
        }

        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn dispatch_select_event(item: &HtmlElement, selected: bool) {
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &wasm_bindgen::JsValue::from_str("selected"), &wasm_bindgen::JsValue::from_bool(selected)).ok();
    let mut init = web_sys::CustomEventInit::new();
    init.bubbles(true);
    init.detail(&detail);
    if let Ok(ev) = web_sys::CustomEvent::new_with_event_init_dict("canon:list-select", &init) {
        item.dispatch_event(&ev).ok();
    }
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
