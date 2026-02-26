use std::sync::OnceLock;

static POINTERMOVE_REGISTERED: OnceLock<()> = OnceLock::new();
static POINTERUP_REGISTERED: OnceLock<()> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
pub fn register_pointer_listener<F>(handler: F) where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    if POINTERMOVE_REGISTERED.set(()).is_err() { return; }
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    leptos::task::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let closure = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                    move |ev: web_sys::PointerEvent| { handler(ev); },
                );
                let _ = doc.add_event_listener_with_callback_and_bool(
                    "pointermove",
                    closure.as_ref().unchecked_ref(),
                    true,
                );
                closure.forget();
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn register_pointerup_listener<F>(handler: F) where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    if POINTERUP_REGISTERED.set(()).is_err() { return; }
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    leptos::task::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let closure = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                    move |ev: web_sys::PointerEvent| { handler(ev); },
                );
                let _ = doc.add_event_listener_with_callback_and_bool(
                    "pointerup",
                    closure.as_ref().unchecked_ref(),
                    true,
                );
                closure.forget();
            }
        }
    });
}
