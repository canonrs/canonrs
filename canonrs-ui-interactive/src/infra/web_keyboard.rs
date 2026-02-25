#[cfg(target_arch = "wasm32")]
pub fn register_keyboard_shortcuts<F1, F2, F3>(
    on_undo: F1,
    on_redo: F2,
    on_delete: F3,
)
where
    F1: Fn() + 'static,
    F2: Fn() + 'static,
    F3: Fn() + 'static,
{
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    leptos::task::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                    move |ev: web_sys::KeyboardEvent| {
                        let ctrl = ev.ctrl_key() || ev.meta_key();
                        let shift = ev.shift_key();
                        let key = ev.key();
                        if ctrl && !shift && key == "z" {
                            ev.prevent_default();
                            on_undo();
                        }
                        if ctrl && shift && key == "z" {
                            ev.prevent_default();
                            on_redo();
                        }
                        if (key == "Delete" || key == "Backspace") && !ctrl {
                            ev.prevent_default();
                            on_delete();
                        }
                    },
                );
                let _ = doc.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    });
}
