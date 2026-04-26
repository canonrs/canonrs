//! OverlayStack — registry global, stack manager, event delegation
//! 1 listener global para todos os overlays — não N por instância

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

// ---------------------------------------------------------------------------
// Stack entry
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct OverlayEntry {
    pub uid:     String,
    pub kind:    &'static str,
    pub z_index: u32,
}

// ---------------------------------------------------------------------------
// Global state (WASM = single-threaded)
// ---------------------------------------------------------------------------

type ClickCb   = Box<dyn Fn(&Element)>;
type KeydownCb = Box<dyn Fn(&web_sys::KeyboardEvent)>;

thread_local! {
    static STACK:            RefCell<Vec<OverlayEntry>>           = RefCell::new(Vec::new());
    static CLICK_REGISTRY:   RefCell<Vec<(String, ClickCb)>>     = RefCell::new(Vec::new());
    static KEYDOWN_REGISTRY: RefCell<Vec<(String, KeydownCb)>>   = RefCell::new(Vec::new());
    static LISTENERS_INIT:   RefCell<bool>                        = RefCell::new(false);
}

const BASE_Z: u32 = 1000;

// ---------------------------------------------------------------------------
// Stack API
// ---------------------------------------------------------------------------

pub fn push(uid: &str, kind: &'static str) -> u32 {
    STACK.with(|s| {
        let mut stack = s.borrow_mut();
        stack.retain(|e| e.uid != uid);
        let z = BASE_Z + (stack.len() as u32 * 10);
        stack.push(OverlayEntry { uid: uid.to_string(), kind, z_index: z });
        z
    })
}

pub fn pop(uid: &str) {
    STACK.with(|s| s.borrow_mut().retain(|e| e.uid != uid));
    // libera scroll_lock apenas se stack vazio
    if stack_empty() {
        crate::runtime::state::set_scroll_lock(false);
    }
}

pub fn top() -> Option<OverlayEntry> {
    STACK.with(|s| s.borrow().last().cloned())
}

pub fn is_top(uid: &str) -> bool {
    top().map(|e| e.uid == uid).unwrap_or(false)
}

pub fn z_for(uid: &str) -> Option<u32> {
    STACK.with(|s| s.borrow().iter().find(|e| e.uid == uid).map(|e| e.z_index))
}

pub fn stack_empty() -> bool {
    STACK.with(|s| s.borrow().is_empty())
}

// ---------------------------------------------------------------------------
// Registry API
// ---------------------------------------------------------------------------

pub fn register_click(uid: &str, cb: impl Fn(&Element) + 'static) {
    CLICK_REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        reg.retain(|(id, _)| id != uid);
        reg.push((uid.to_string(), Box::new(cb)));
    });
}

pub fn register_keydown(uid: &str, cb: impl Fn(&web_sys::KeyboardEvent) + 'static) {
    KEYDOWN_REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        reg.retain(|(id, _)| id != uid);
        reg.push((uid.to_string(), Box::new(cb)));
    });
}

pub fn unregister(uid: &str) {
    CLICK_REGISTRY.with(|r|   r.borrow_mut().retain(|(id, _)| id != uid));
    KEYDOWN_REGISTRY.with(|r| r.borrow_mut().retain(|(id, _)| id != uid));
    pop(uid);
}

// ---------------------------------------------------------------------------
// Global listeners — init único para TODOS os overlays
// ---------------------------------------------------------------------------

pub fn ensure_global_listeners() {
    LISTENERS_INIT.with(|init| {
        if *init.borrow() { return; }
        *init.borrow_mut() = true;

        // 1 click listener global
        // - se stack vazio: despacha para TODOS (trigger pode abrir qualquer dialog)
        // - se stack ativo: despacha apenas para o top (evita interação com dialogs abaixo)
        {
            let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(|e: web_sys::MouseEvent| {
                let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
                let top = top();
                CLICK_REGISTRY.with(|r| {
                    let reg = r.borrow();
                    match top {
                        Some(t) => {
                            // stack ativo: só o top recebe
                            if let Some((_, cb)) = reg.iter().find(|(id, _)| *id == t.uid) {
                                cb(&target);
                            }
                        }
                        None => {
                            // stack vazio: todos recebem (para detectar trigger)
                            for (_, cb) in reg.iter() {
                                cb(&target);
                            }
                        }
                    }
                });
            });
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }

        // 1 keydown listener global — despacha apenas para o top do stack
        {
            let cb = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(|e: web_sys::KeyboardEvent| {
                let Some(top) = top() else { return };
                KEYDOWN_REGISTRY.with(|r| {
                    let reg = r.borrow();
                    if let Some((_, cb)) = reg.iter().find(|(id, _)| *id == top.uid) {
                        cb(&e);
                    }
                });
            });
            if let Some(win) = web_sys::window() {
                let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            }
            cb.forget();
        }
    });
}

// ---------------------------------------------------------------------------
// z-index — aplica no portal do overlay
// ---------------------------------------------------------------------------

pub fn apply_z(uid: &str, overlay_sel: &str, content_sel: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let Some(z) = z_for(uid) else { return };
    if let Ok(Some(el)) = doc.query_selector(overlay_sel) {
        let _ = el.set_attribute("style", &format!("z-index:{}", z));
    }
    if let Ok(Some(el)) = doc.query_selector(content_sel) {
        let _ = el.set_attribute("style", &format!("z-index:{}", z + 1));
    }
}
