use leptos::prelude::*;

pub fn use_column_resize<F>(
    container_id: String,
    enabled: Signal<bool>,
    on_resize: F,
) where
    F: Fn(String, u32) + 'static + Clone,
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

            let doc = document();
            let container_id_static: &'static str = Box::leak(container_id.clone().into_boxed_str());
            if let Some(container) = doc.get_element_by_id(container_id_static) {
                if container.get_attribute("data-resize-hook-attached").as_deref() == Some("1") {
                    return;
                }

                let _ = container.set_attribute("data-resize-hook-attached", "1");

                let on_resize_clone = on_resize.clone();
                let closure = Closure::wrap(Box::new(move |ev: leptos::web_sys::Event| {
                    if let (Ok(col_val), Ok(width_val)) = (
                        js_sys::Reflect::get(&ev, &JsValue::from_str("colId")),
                        js_sys::Reflect::get(&ev, &JsValue::from_str("width")),
                    ) {
                        if let (Some(col), Some(width)) = (col_val.as_string(), width_val.as_f64()) {
                            on_resize_clone(col, width as u32);
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = container.add_event_listener_with_callback(
                    "canon:resize:move",
                    closure.as_ref().unchecked_ref(),
                );
                closure.forget();

                // Cleanup quando effect é destruído
                on_cleanup(move || {
                    if let Some(container) = document().get_element_by_id(container_id_static) {
                        let _ = container.remove_attribute("data-resize-hook-attached");
                    }
                });
            }
        }
    });
}
