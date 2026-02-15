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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = clipboard)]
    #[wasm_bindgen(thread_local_v2)]
    static CLIPBOARD: Clipboard;

    type Clipboard;

    #[wasm_bindgen(method, js_name = writeText)]
    fn write_text(this: &Clipboard, text: &str) -> js_sys::Promise;
}

#[cfg(feature = "hydrate")]
pub fn register() {
    register_behavior("data-copy-button", Box::new(|element_id, _state| {
        let Some(btn) = document().get_element_by_id(element_id) else {
            return Ok(());
        };
        setup_copy_button(&btn)?;
        Ok(())
    }));
}

#[cfg(feature = "hydrate")]
fn setup_copy_button(btn: &Element) -> BehaviorResult<()> {
    if btn.get_attribute("data-copy-attached").as_deref() == Some("1") {
        return Ok(());
    }
    let _ = btn.set_attribute("data-copy-attached", "1");
    let _ = btn.set_attribute("data-state", "idle");

    let btn_clone = btn.clone();

    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
        // Get text from data-copy-text or from data-copy-target element
        let text = btn_clone.get_attribute("data-copy-text")
            .filter(|t| !t.is_empty())
            .or_else(|| {
                btn_clone.get_attribute("data-copy-target").and_then(|target_id| {
                    document().get_element_by_id(&target_id)
                        .and_then(|el| el.text_content())
                })
            })
            .unwrap_or_default();

        CLIPBOARD.with(|clip| {
            let _ = clip.write_text(&text);
        });

        let _ = btn_clone.set_attribute("data-state", "copied");

        // Reset after 2s
        let btn_reset = btn_clone.clone();
        let timeout = Closure::once(Box::new(move || {
            let _ = btn_reset.set_attribute("data-state", "idle");
        }) as Box<dyn FnOnce()>);

        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout.as_ref().unchecked_ref(),
                2000,
            );
        timeout.forget();
    }) as Box<dyn FnMut(_)>);

    btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .map_err(|_| canonrs_shared::BehaviorError::JsError { message: "listener failed".into() })?;
    closure.forget();
    Ok(())
}

#[cfg(not(feature = "hydrate"))]
pub fn register() {}
