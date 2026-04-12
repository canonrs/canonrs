pub mod registry;
pub mod scanner;
pub mod dispatcher;
pub mod observer;

use web_sys::Element;

pub fn init_element(el: &Element) {
    if !registry::should_init(el) { return; }
    dispatcher::dispatch(el);
}

pub fn scan_and_init() {
    for el in scanner::query("[data-rs-interaction]") {
        init_element(&el);
    }
    observer::observe();
}
