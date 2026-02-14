use leptos::prelude::*;

pub fn use_column_reorder<F>(
    container_id: String,
    enabled: Signal<bool>,
    on_reorder: F,
) where
    F: Fn(String, String) + 'static + Clone,
{
    let on_reorder_outer = on_reorder.clone();

    create_effect(move |_| {
        if !enabled.get() {
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::{closure::Closure, JsCast, JsValue};
            use leptos::leptos_dom::helpers::document;
            use leptos::prelude::set_timeout;

            let on_reorder_for_timeout = on_reorder_outer.clone();
            let container_id_clone = container_id.clone();

            set_timeout(
                move || {
                    let doc = document();
                    if let Some(container) = doc.get_element_by_id(&container_id_clone) {
                        if container.get_attribute("data-reorder-hook-attached").as_deref() == Some("1") {
                            return;
                        }

                        let _ = container.set_attribute("data-reorder-hook-attached", "1");
                        let on_reorder_clone = on_reorder_for_timeout.clone();
                        let container_id_cleanup = container_id_clone.clone();

                        let closure = Closure::wrap(Box::new(move |ev: leptos::web_sys::Event| {
                            if let (Ok(from_val), Ok(to_val)) = (
                                js_sys::Reflect::get(&ev, &JsValue::from_str("dragFrom")),
                                js_sys::Reflect::get(&ev, &JsValue::from_str("dragTo")),
                            ) {
                                if let (Some(from), Some(to)) = (from_val.as_string(), to_val.as_string()) {
                                    on_reorder_clone(from, to);
                                }
                            }
                        }) as Box<dyn FnMut(_)>);

                        let _ = container.add_event_listener_with_callback(
                            "canon:reorder",
                            closure.as_ref().unchecked_ref(),
                        );
                        closure.forget();

                        on_cleanup(move || {
                            if let Some(c) = document().get_element_by_id(&container_id_cleanup) {
                                let _ = c.remove_attribute("data-reorder-hook-attached");
                            }
                        });
                    }
                },
                std::time::Duration::from_millis(150),
            );
        }
    });
}
