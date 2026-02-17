#[cfg(feature = "hydrate")]
use super::register_behavior;
#[cfg(feature = "hydrate")]
use canonrs_shared::BehaviorResult;
#[cfg(feature = "hydrate")]
use leptos::leptos_dom::helpers::document;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys::Element;

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-list", Box::new(|element_id, _state| {
        web_sys::console::log_1(&format!("🔵 List behavior init: {}", element_id).into());
        
        let Some(list) = document().get_element_by_id(element_id) else {
            web_sys::console::log_1(&"❌ List element not found".into());
            return Ok(());
        };
        
        setup_list(&list)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_list(list: &Element) -> BehaviorResult<()> {
    if list.get_attribute("data-list-attached").as_deref() == Some("1") {
        web_sys::console::log_1(&"⚠️ List already attached".into());
        return Ok(());
    }
    let _ = list.set_attribute("data-list-attached", "1");

    let items = list.query_selector_all("[data-list-item-content][data-selectable]")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query items".into() })?;

    web_sys::console::log_1(&format!("📋 Found {} selectable items", items.length()).into());

    for i in 0..items.length() {
        if let Some(item) = items.item(i) {
            setup_list_item(&item)?;
        }
    }

    Ok(())
}

#[cfg(feature = "hydrate")]
fn setup_list_item(item: &web_sys::Node) -> BehaviorResult<()> {
    let item_el: web_sys::HtmlElement = item.clone().dyn_into()
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "cast item".into() })?;

    if item_el.has_attribute("data-disabled") {
        return Ok(());
    }

    web_sys::console::log_1(&"✅ Attaching handlers to item".into());

    let item_clone = item_el.clone();
    let on_click = Closure::wrap(Box::new(move |_: web_sys::Event| {
        web_sys::console::log_1(&"🖱️ Click detected!".into());
        toggle_selection(&item_clone);
    }) as Box<dyn FnMut(_)>);

    item_el.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "click listener".into() })?;
    on_click.forget();

    let item_clone2 = item_el.clone();
    let on_keydown = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let key = e.key();
        if key == "Enter" || key == " " {
            web_sys::console::log_1(&format!("⌨️ Key: {}", key).into());
            e.prevent_default();
            toggle_selection(&item_clone2);
        }
    }) as Box<dyn FnMut(_)>);

    item_el.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "keydown listener".into() })?;
    on_keydown.forget();

    Ok(())
}

#[cfg(feature = "hydrate")]
fn toggle_selection(item: &web_sys::HtmlElement) {
    let is_selected = item.has_attribute("data-selected");
    
    web_sys::console::log_1(&format!("🔄 Toggle selection: {} -> {}", is_selected, !is_selected).into());
    
    if is_selected {
        let _ = item.remove_attribute("data-selected");
        let _ = item.set_attribute("aria-selected", "false");
    } else {
        let _ = item.set_attribute("data-selected", "");
        let _ = item.set_attribute("aria-selected", "true");
    }

    dispatch_select_event(item, !is_selected);
}

#[cfg(feature = "hydrate")]
fn dispatch_select_event(item: &web_sys::HtmlElement, selected: bool) {
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &JsValue::from_str("selected"), &JsValue::from_bool(selected)).ok();

    let f = js_sys::Function::new_with_args(
        "el,detail",
        "el.dispatchEvent(new CustomEvent('canon:list-select', {bubbles:true, detail:detail}));"
    );
    f.call2(&JsValue::NULL, item, &detail).ok();
    
    web_sys::console::log_1(&"📡 Event dispatched: canon:list-select".into());
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
