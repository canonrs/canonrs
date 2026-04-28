pub mod registry;
pub mod scanner;
pub mod dispatcher;
pub mod observer;

use web_sys::Element;

pub fn init_element(el: &Element) {
    // inicializa o proprio elemento se tem data-rs-interaction
    if el.has_attribute("data-rs-interaction") {
        if registry::should_init(el) {
            dispatcher::dispatch(el);
            // se o proprio elemento foi inicializado, escaneia filhos
            // caso contrario, ja foi processado — skip query_within
            scan_children(el);
        }
        return;
    }
    // elemento nao tem data-rs-interaction — escaneia filhos diretamente
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
