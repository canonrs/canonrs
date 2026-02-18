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
    register_behavior("data-toolbar", Box::new(|element_id, _state| {
        let Some(toolbar) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        
        setup_toolbar(&toolbar)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_toolbar(toolbar: &Element) -> BehaviorResult<()> {
    if toolbar.get_attribute("data-toolbar-attached").as_deref() == Some("1") {
        return Ok(());
    }
    toolbar.set_attribute("data-toolbar-attached", "1").ok();

    let orientation = toolbar.get_attribute("aria-orientation")
        .unwrap_or_else(|| "horizontal".to_string());
    let is_horizontal = orientation == "horizontal";

    let items = toolbar.query_selector_all("button, input, select, [tabindex='0']")
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "query items".into() })?;

    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            if let Ok(item) = node.dyn_into::<Element>() {
                if i == 0 {
                    item.set_attribute("tabindex", "0").ok();
                } else {
                    item.set_attribute("tabindex", "-1").ok();
                }
            }
        }
    }

    let toolbar_clone = toolbar.clone();
    let on_keydown = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let key = e.key();
        
        let should_handle = match (is_horizontal, key.as_str()) {
            (true, "ArrowLeft" | "ArrowRight" | "Home" | "End") => true,
            (false, "ArrowUp" | "ArrowDown" | "Home" | "End") => true,
            _ => false,
        };

        if !should_handle {
            return;
        }

        e.prevent_default();

        let items = toolbar_clone.query_selector_all("button, input, select, [tabindex='0'], [tabindex='-1']")
            .ok().unwrap();
        
        let total = items.length();
        if total == 0 { return; }

        let active_element = document().active_element();
        let mut current_index = 0;
        
        for i in 0..total {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    if active_element.as_ref() == Some(&item) {
                        current_index = i;
                        break;
                    }
                }
            }
        }

        let new_index = match key.as_str() {
            "ArrowLeft" | "ArrowUp" => {
                if current_index > 0 { current_index - 1 } else { current_index }
            },
            "ArrowRight" | "ArrowDown" => {
                if current_index < total - 1 { current_index + 1 } else { current_index }
            },
            "Home" => 0,
            "End" => total - 1,
            _ => current_index,
        };

        for i in 0..total {
            if let Some(node) = items.item(i) {
                if let Ok(item) = node.dyn_into::<Element>() {
                    if i == new_index {
                        item.set_attribute("tabindex", "0").ok();
                        if let Ok(html_el) = item.dyn_into::<web_sys::HtmlElement>() {
                            html_el.focus().ok();
                        }
                    } else {
                        item.set_attribute("tabindex", "-1").ok();
                    }
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    toolbar.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "keydown".into() })?;
    on_keydown.forget();

    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
