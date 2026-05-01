pub mod registry;
pub mod scanner;
pub mod dispatcher;
pub mod observer;

use web_sys::Element;

pub fn init_element(el: &Element) {
    if el.has_attribute("data-rs-interaction") {
        if registry::should_init(el) {
            dispatcher::dispatch(el);
            scan_children(el);
        }
        return;
    }
    scan_children(el);
}

fn scan_children(el: &Element) {
    for child in scanner::query_within(el, "[data-rs-interaction]") {
        if registry::should_init(&child) {
            dispatcher::dispatch(&child);
        }
    }
}

pub fn scan_and_init() {
    for el in scanner::query("[data-rs-interaction]") {
        init_element(&el);
    }
    observer::observe();
}
