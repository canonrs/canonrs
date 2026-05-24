//! Listeners — owned event listener registry
//!
//! Cada listener retorna um ListenerHandle.
//! Drop do handle remove o listener automaticamente.
//! Ownership por namespace (uid) — cleanup_uid() remove todos de um componente.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

fn next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

struct ListenerEntry {
    target:  EventTarget,
    event:   String,
    capture: bool,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl ListenerEntry {
    fn remove(&self) {
        let _ = self.target.remove_event_listener_with_callback_and_bool(
            &self.event,
            self.closure.as_ref().unchecked_ref(),
            self.capture,
        );
    }
}

thread_local! {
    // namespace -> (id -> entry)
    static REGISTRY: RefCell<HashMap<String, HashMap<usize, ListenerEntry>>> =
        RefCell::new(HashMap::new());
}

fn register(ns: &str, target: EventTarget, event: &str, capture: bool, closure: Closure<dyn FnMut(web_sys::Event)>) -> usize {
    let id = next_id();
    REGISTRY.with(|r| {
        r.borrow_mut()
            .entry(ns.to_string())
            .or_default()
            .insert(id, ListenerEntry { target, event: event.to_string(), capture, closure });
    });
    // Track in ownership graph
    if !ns.is_empty() && ns != "global" {
        super::ownership::track_listener(ns);
    }
    id
}

/// Opções de listener
pub struct ListenOpts {
    pub capture: bool,
    pub passive: bool,
}

impl Default for ListenOpts {
    fn default() -> Self { Self { capture: false, passive: false } }
}

/// Listener em elemento — retorna id para cleanup manual
pub fn listen<F>(ns: &str, el: &Element, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    listen_opts(ns, el.unchecked_ref(), event, ListenOpts::default(), cb)
}

/// Listener com opcoes (capture)
pub fn listen_opts<F>(ns: &str, target: &EventTarget, event: &str, opts: ListenOpts, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    let closure = Closure::wrap(Box::new(cb) as Box<dyn FnMut(web_sys::Event)>);
    let _ = target.add_event_listener_with_callback_and_bool(
        event,
        closure.as_ref().unchecked_ref(),
        opts.capture,
    );
    register(ns, target.clone(), event, opts.capture, closure)
}

/// Listener no document
pub fn listen_document<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return 0 };
    listen_opts(ns, doc.unchecked_ref(), event, ListenOpts::default(), cb)
}

/// Listener na window
pub fn listen_window<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    let Some(win) = web_sys::window() else { return 0 };
    listen_opts(ns, win.unchecked_ref(), event, ListenOpts::default(), cb)
}

/// Listener MouseEvent tipado
pub fn on_click<F>(ns: &str, el: &Element, mut cb: F) -> usize
where F: FnMut(web_sys::MouseEvent) + 'static
{
    listen(ns, el, "click", move |e: web_sys::Event| {
        if let Ok(me) = e.dyn_into::<web_sys::MouseEvent>() { cb(me); }
    })
}

/// Listener MouseEvent no document
pub fn on_click_document<F>(ns: &str, mut cb: F) -> usize
where F: FnMut(web_sys::MouseEvent) + 'static
{
    listen_document(ns, "click", move |e: web_sys::Event| {
        if let Ok(me) = e.dyn_into::<web_sys::MouseEvent>() { cb(me); }
    })
}

/// Listener KeyboardEvent no document
pub fn on_keydown_document<F>(ns: &str, mut cb: F) -> usize
where F: FnMut(web_sys::KeyboardEvent) + 'static
{
    listen_document(ns, "keydown", move |e: web_sys::Event| {
        if let Ok(ke) = e.dyn_into::<web_sys::KeyboardEvent>() { cb(ke); }
    })
}

/// Listener KeyboardEvent na window
pub fn on_keydown_window<F>(ns: &str, mut cb: F) -> usize
where F: FnMut(web_sys::KeyboardEvent) + 'static
{
    listen_window(ns, "keydown", move |e: web_sys::Event| {
        if let Ok(ke) = e.dyn_into::<web_sys::KeyboardEvent>() { cb(ke); }
    })
}

/// Listener PointerEvent no document
pub fn on_pointer_document<F>(ns: &str, event: &str, mut cb: F) -> usize
where F: FnMut(web_sys::PointerEvent) + 'static
{
    listen_document(ns, event, move |e: web_sys::Event| {
        if let Ok(pe) = e.dyn_into::<web_sys::PointerEvent>() { cb(pe); }
    })
}

/// Remove listener por id
pub fn remove(id: usize) {
    REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        for ns_map in reg.values_mut() {
            if let Some(entry) = ns_map.remove(&id) {
                entry.remove();
                return;
            }
        }
    });
}

/// Remove todos os listeners de um namespace (uid/componente)
pub fn cleanup(ns: &str) {
    REGISTRY.with(|r| {
        if let Some(ns_map) = r.borrow_mut().remove(ns) {
            for (_, entry) in ns_map {
                entry.remove();
            }
        }
    });
}

/// GC — remove listeners de elementos desconectados
/// Total de listeners ativos no registry
pub fn active_count() -> usize {
    REGISTRY.with(|r| {
        r.borrow().values().map(|m| m.len()).sum()
    })
}

/// Namespaces (uids) ativos no registry
pub fn namespaces() -> Vec<String> {
    REGISTRY.with(|r| r.borrow().keys().cloned().collect())
}

/// Listeners em elementos desconectados do DOM (orphans)
pub fn orphan_count() -> usize {
    use wasm_bindgen::JsCast;
    REGISTRY.with(|r| {
        r.borrow().values().flat_map(|m| m.values()).filter(|entry| {
            entry.target.dyn_ref::<Element>()
                .map(|el| !el.is_connected())
                .unwrap_or(false)
        }).count()
    })
}

pub fn gc() {
    REGISTRY.with(|r| {
        let mut reg = r.borrow_mut();
        for ns_map in reg.values_mut() {
            ns_map.retain(|_, entry| {
                if let Some(el) = entry.target.dyn_ref::<Element>() {
                    if !el.is_connected() {
                        entry.remove(); // remove do browser antes de dropar
                        return false;
                    }
                    true
                } else {
                    true // document/window — nunca remover
                }
            });
        }
        reg.retain(|_, ns_map| !ns_map.is_empty());
    });
}
