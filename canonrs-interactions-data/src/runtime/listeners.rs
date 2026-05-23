//! Data Runtime — listeners
//! uid-driven wrapper over canonrs_interactions_core::runtime::listeners
//!
//! Todas as APIs lêem data-rs-uid do elemento automaticamente.
//! Módulos NÃO precisam pegar uid manualmente.

use web_sys::Element;
use canonrs_interactions_core::runtime::listeners as core;

/// Extrai uid de data-rs-uid do elemento.
/// Retorna string vazia se ausente (listener ainda registrado sem namespace).
fn uid(el: &Element) -> String {
    el.get_attribute("data-rs-uid").unwrap_or_default()
}

/// Listener em elemento — namespace = uid do elemento
pub fn listen_uid<F>(el: &Element, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    core::listen(&uid(el), el, event, cb)
}

/// Listener em elemento com uid explícito (para casos onde uid vem do root, não do target)
pub fn listen<F>(ns: &str, el: &Element, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    core::listen(ns, el, event, cb)
}

/// Listener no document com uid explícito
pub fn listen_document<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    core::listen_document(ns, event, cb)
}

/// Listener na window com uid explícito
pub fn listen_window<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    core::listen_window(ns, event, cb)
}

/// Listener capture no document (ex: click capture para fechar menus)
pub fn listen_document_capture<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(web_sys::Event) + 'static
{
    core::listen_opts(ns, &web_sys::window().unwrap().document().unwrap().into(),
        event,
        core::ListenOpts { capture: true, passive: false },
        cb,
    )
}

/// Remove todos os listeners de um namespace (uid)
pub fn cleanup(ns: &str) {
    core::cleanup(ns);
}
