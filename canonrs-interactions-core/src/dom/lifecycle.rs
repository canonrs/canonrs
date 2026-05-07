//! Lifecycle — init guard + reinit support
use web_sys::Element;

pub fn is_initialized(el: &Element) -> bool {
    el.get_attribute("data-rs-initialized").as_deref() == Some("true")
}

pub fn needs_reinit(el: &Element) -> bool {
    el.has_attribute("data-rs-reinit")
}

pub fn clear_reinit(el: &Element) {
    let _ = el.remove_attribute("data-rs-reinit");
}

pub fn mark_initialized(el: &Element) {
    let _ = el.set_attribute("data-rs-initialized", "true");
}

/// Retorna true se o elemento deve ser inicializado.
/// Suporta reinit via data-rs-reinit (ex: islands com hydration).
pub fn init_guard(el: &Element) -> bool {
    if is_initialized(el) {
        if needs_reinit(el) {
            clear_reinit(el);
            return true;
        }
        return false;
    }
    mark_initialized(el);
    true
}
