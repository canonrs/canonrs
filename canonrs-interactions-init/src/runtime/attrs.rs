//! Attrs — helpers para leitura de atributos DOM
use web_sys::Element;

pub fn get_str(el: &Element, attr: &str) -> String {
    el.get_attribute(attr).unwrap_or_default()
}

pub fn get_bool(el: &Element, attr: &str) -> bool {
    el.get_attribute(attr).as_deref() == Some("true")
}
