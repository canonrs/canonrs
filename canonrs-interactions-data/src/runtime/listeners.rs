//! Data Runtime — listeners
//! uid-driven wrapper over canonrs_interactions_core::runtime::listeners
//!
//! Importa tipos web_sys via canonrs_interactions_core::web para garantir
//! unificação de tipos entre crates. NÃO importar web_sys diretamente aqui.

pub use canonrs_interactions_core::runtime::listeners::{
    listen,
    listen_opts,
    listen_document,
    listen_window,
    cleanup,
    ListenOpts,
};
use canonrs_interactions_core::runtime::listeners as core;
use canonrs_interactions_core::web::{Element, Event};

/// Extrai uid de data-rs-uid do elemento.
fn uid(el: &Element) -> String {
    el.get_attribute("data-rs-uid").unwrap_or_default()
}

/// Listener em elemento — namespace = uid do elemento
pub fn listen_uid<F>(el: &Element, event: &str, cb: F) -> usize
where F: FnMut(Event) + 'static
{
    let ns = uid(el);
    listen(&ns, el, event, cb)
}

/// Listener capture no document (ex: click capture para fechar menus)
pub fn listen_document_capture<F>(ns: &str, event: &str, cb: F) -> usize
where F: FnMut(Event) + 'static
{
    core::listen_opts(
        ns,
        &web_sys::window().unwrap().document().unwrap().into(),
        event,
        core::ListenOpts { capture: true, passive: false },
        cb,
    )
}
