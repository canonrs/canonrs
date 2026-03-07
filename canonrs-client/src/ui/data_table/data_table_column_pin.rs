use leptos::prelude::*;
use super::PinPosition;

pub fn use_column_pin<F>(
    container_id: String,
    enabled: Signal<bool>,
    on_pin: F,
) where
    F: Fn(String, PinPosition) + 'static + Clone,
{
    create_effect(move |_| {
        enabled.track();

        if !enabled.get() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::{closure::Closure, JsCast, JsValue};
            use leptos::leptos_dom::helpers::document;
            use leptos::web_sys::{Element, Node, Event, EventTarget};

            let doc = document();

            if let Some(container) = doc.get_element_by_id(container_id.as_str()) {
                if container.get_attribute("data-pin-hook-attached").as_deref() == Some("1") {
                    return;
                }

                let _ = container.set_attribute("data-pin-hook-attached", "1");

                let on_pin_clone = on_pin.clone();
                let container_clone = container.clone();
                let container_id_cleanup = container_id.clone();

                let click_closure = Closure::wrap(Box::new(move |e: leptos::web_sys::MouseEvent| {
                    if let Some(target) = e.target() {
                        if let Ok(el) = target.dyn_into::<Element>() {
                            let button = if el.has_attribute("data-pin-button") {
                                Some(el)
                            } else {
                                el.closest("[data-pin-button]").ok().flatten()
                            };

                            if let Some(btn) = button {
                                e.prevent_default();
                                e.stop_propagation();

                                let col_id = match Element::get_attribute(&btn, "data-pin-col-id") {
                                    Some(id) => id,
                                    None => {
                                        leptos::logging::log!("pin: no col_id found on button");
                                        return;
                                    }
                                };
                                leptos::logging::log!("pin clicked: col_id={}", col_id);

                                // Ciclo: unpinned → pinned-left → pinned-right → unpinned
                                let current = Element::get_attribute(&btn, "data-pin-state")
                                    .unwrap_or_else(|| "unpinned".to_string());

                                let (new_state, new_icon, pin_position) = match current.as_str() {
                                    "unpinned"    => ("pinned-left",  "⬅📌", PinPosition::Left),
                                    "pinned-left" => ("pinned-right", "📌➡", PinPosition::Right),
                                    _             => ("unpinned",     "📍",   PinPosition::None),
                                };

                                let _ = Element::set_attribute(&btn, "data-pin-state", new_state);
                                let btn_node: Node = btn.clone().dyn_into::<Node>().unwrap();
                                Node::set_text_content(&btn_node, Some(new_icon));

                                on_pin_clone(col_id.clone(), pin_position);

                                if let Ok(evt) = Event::new("canon:pin:toggle") {
                                    let _ = js_sys::Reflect::set(&evt, &JsValue::from_str("colId"), &JsValue::from_str(&col_id));
                                    let _ = js_sys::Reflect::set(&evt, &JsValue::from_str("pinState"), &JsValue::from_str(new_state));
                                    let target: EventTarget = container_clone.clone().dyn_into().unwrap();
                                    let _ = EventTarget::dispatch_event(&target, &evt);
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = container.add_event_listener_with_callback_and_bool(
                    "click",
                    click_closure.as_ref().unchecked_ref(),
                    true,
                );
                click_closure.forget();

                on_cleanup(move || {
                    if let Some(c) = document().get_element_by_id(container_id_cleanup.as_str()) {
                        let _ = c.remove_attribute("data-pin-hook-attached");
                    }
                });
            }
        }
    });
}
