//! Lifecycle — init guard para evitar dupla inicialização
use web_sys::Element;

pub fn init_guard(el: &Element) -> bool {
    if el.get_attribute("data-rs-initialized").as_deref() == Some("true") {
        return false;
    }
    let _ = el.set_attribute("data-rs-initialized", "true");
    true
}
