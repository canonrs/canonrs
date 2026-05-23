//! Timers — owned timeout/RAF handles
//!
//! timeout() e raf() retornam handle i32 para cancelamento.
//! Closures once são liberadas pelo JS GC após execução.

use wasm_bindgen::prelude::*;

/// Executa cb após delay_ms — retorna handle para cancelamento
pub fn timeout<F>(delay_ms: i32, cb: F) -> i32
where F: FnOnce() + 'static
{
    let closure = Closure::once(Box::new(cb) as Box<dyn FnOnce()>);
    let handle = web_sys::window()
        .and_then(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(), delay_ms
        ).ok())
        .unwrap_or(-1);
    closure.forget(); // once — liberado pelo JS após execução
    handle
}

/// Cancela timeout pelo handle
pub fn cancel_timeout(handle: i32) {
    if handle >= 0 {
        if let Some(win) = web_sys::window() {
            win.clear_timeout_with_handle(handle);
        }
    }
}

/// Executa cb no próximo animation frame — retorna handle
pub fn raf<F>(cb: F) -> i32
where F: FnOnce() + 'static
{
    let closure = Closure::once(Box::new(cb) as Box<dyn FnOnce()>);
    let handle = web_sys::window()
        .and_then(|w| w.request_animation_frame(closure.as_ref().unchecked_ref()).ok())
        .unwrap_or(-1);
    closure.forget();
    handle
}

/// Cancela RAF pelo handle
pub fn cancel_raf(handle: i32) {
    if handle >= 0 {
        if let Some(win) = web_sys::window() {
            let _ = win.cancel_animation_frame(handle);
        }
    }
}

/// Próximo frame visual (16ms)
pub fn next_frame<F>(cb: F) -> i32
where F: FnOnce() + 'static
{
    timeout(16, cb)
}

/// Após transição (32ms — dois frames)
pub fn after_transition<F>(cb: F) -> i32
where F: FnOnce() + 'static
{
    timeout(32, cb)
}

/// Após transição com duração customizada
pub fn after_duration<F>(duration_ms: i32, cb: F) -> i32
where F: FnOnce() + 'static
{
    timeout(duration_ms + 16, cb)
}
