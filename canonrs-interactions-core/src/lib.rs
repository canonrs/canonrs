#![deny(warnings)]
//! canonrs-interactions-core — kernel compartilhado entre todos os grupos de interação

pub mod dom;
pub mod behavior;
pub mod integration;
pub mod runtime;

/// Re-exports de web_sys para garantir unificação de tipos entre crates.
/// Crates filhos devem importar tipos web_sys daqui, não diretamente.
pub mod web {
    pub use web_sys::{
        Element, HtmlElement, HtmlInputElement,
        Event, MouseEvent, KeyboardEvent, PointerEvent,
        EventTarget, Document, Window,
        CustomEvent, CustomEventInit,
        HtmlCanvasElement, CanvasRenderingContext2d,
        Node, NodeList,
    };
}
