#![deny(warnings)]
//! canonrs-runtime — lifecycle engine compartilhado
//! Usado por todos os crates de interação e pelo bundle

pub mod registry;
pub mod scanner;
pub mod observer;

use web_sys::Element;

/// Trait que o bundle implementa para despachar init por grupo
pub trait Dispatcher: 'static {
    fn dispatch(&self, el: &Element);
}

use std::cell::RefCell;

thread_local! {
    static DISPATCHER: RefCell<Option<Box<dyn Dispatcher>>> = RefCell::new(None);
}

/// Registra o dispatcher — chamado pelo bundle no startup
pub fn register_dispatcher(d: impl Dispatcher) {
    DISPATCHER.with(|r| {
        *r.borrow_mut() = Some(Box::new(d));
    });
}

pub fn init_element(el: &Element) {
    if el.has_attribute("data-rs-interaction") {
        if registry::should_init(el) {
            DISPATCHER.with(|r| {
                if let Some(ref d) = *r.borrow() {
                    d.dispatch(el);
                }
            });
            scan_children(el);
        }
        return;
    }
    scan_children(el);
}

pub fn scan_children(el: &Element) {
    for child in scanner::query_within(el, "[data-rs-interaction]") {
        if registry::should_init(&child) {
            DISPATCHER.with(|r| {
                if let Some(ref d) = *r.borrow() {
                    d.dispatch(&child);
                }
            });
        }
    }
}

pub fn scan_and_init() {
    for el in scanner::query("[data-rs-interaction]") {
        init_element(&el);
    }
    observer::observe();
}
