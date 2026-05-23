#![deny(warnings)]
//! canonrs-interactions-content

pub mod markdown;
pub mod copy_button;


use canonrs_interactions_core::runtime::bootstrap;

/// Registra o grupo content no bootstrap kernel.
pub fn register() {
    bootstrap::register("content", init_content);
}

/// Init subtree — replay-safe, delega para bootstrap kernel.
pub fn init_subtree(root: &web_sys::Element) {
    bootstrap::init_subtree(root);
}
pub fn init_content(el: web_sys::Element) {
    if el.has_attribute("data-rs-markdown") { markdown::init(el.clone()); }
    if el.has_attribute("data-rs-copy-button") { copy_button::init(el.clone()); }
}
