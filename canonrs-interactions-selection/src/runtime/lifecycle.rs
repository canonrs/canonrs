//! Lifecycle — init guard delegado ao orchestrador central
//! O registry de uids vive em canonrs-interactions, nao aqui.
//! init_guard aqui e apenas uma dupla verificacao local via DOM state.
use web_sys::Element;
use crate::runtime::state;

/// Retorna true se o componente deve ser inicializado.
/// Usa data-rs-state para verificar — sem atributo separado no DOM.
pub fn init_guard(el: &Element) -> bool {
    if state::has(el, "initialized") { return false; }
    state::add(el, "initialized");
    true
}
