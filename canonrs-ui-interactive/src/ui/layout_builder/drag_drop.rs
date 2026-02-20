use leptos::prelude::*;
use super::types::{BlockDef, all_blocks};

pub fn setup_drag_drop(canvas_id: String, dropped_blocks: RwSignal<Vec<BlockDef>>) {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::{closure::Closure, JsCast};
        use leptos::leptos_dom::helpers::document;
        use leptos::prelude::set_timeout;

        set_timeout(move || {
            let doc = document();
            if let Some(canvas) = doc.get_element_by_id(&canvas_id) {
                if canvas.get_attribute("data-drop-attached").as_deref() == Some("1") {
                    return;
                }
                let _ = canvas.set_attribute("data-drop-attached", "1");

                // dragover — prevent default via JS eval on element
                let over_closure = Closure::wrap(Box::new(move |ev: leptos::web_sys::Event| {
                    ev.prevent_default();
                }) as Box<dyn FnMut(_)>);
                let _ = canvas.add_event_listener_with_callback("dragover", over_closure.as_ref().unchecked_ref());
                over_closure.forget();

                // drop — read data via js_sys eval
                let drop_closure = Closure::wrap(Box::new(move |ev: leptos::web_sys::Event| {
                    ev.prevent_default();
                    // Use js_sys to access dataTransfer
                    let data = js_sys::Reflect::get(&ev, &leptos::wasm_bindgen::JsValue::from_str("dataTransfer"))
                        .ok()
                        .and_then(|dt| {
                            js_sys::Reflect::get(&dt, &leptos::wasm_bindgen::JsValue::from_str("getData"))
                                .ok()
                                .and_then(|get_fn| {
                                    js_sys::Function::from(get_fn)
                                        .call1(&dt, &leptos::wasm_bindgen::JsValue::from_str("text/plain"))
                                        .ok()
                                })
                        })
                        .and_then(|v| v.as_string());

                    if let Some(block_id) = data {
                        if !block_id.is_empty() {
                            if let Some(block) = all_blocks().into_iter().find(|b| b.id == block_id.as_str()) {
                                dropped_blocks.update(|v| v.push(block));
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                let _ = canvas.add_event_listener_with_callback("drop", drop_closure.as_ref().unchecked_ref());
                drop_closure.forget();
            }
        }, std::time::Duration::from_millis(150));
    }
}

pub fn get_drag_id_script(element_id: &str) -> String {
    format!(
        "document.getElementById('{}').addEventListener('dragstart', function(e) {{ e.dataTransfer.setData('text/plain', e.target.closest('[data-builder-block]')?.dataset.builderBlock || ''); }});",
        element_id
    )
}
