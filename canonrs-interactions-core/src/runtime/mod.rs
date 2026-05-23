//! Runtime kernel — centralized ownership for all interaction resources
//!
//! Todos os crates de interação devem usar este módulo.
//! Zero cb.forget() fora deste módulo.

pub mod listeners;
pub mod timers;
pub mod bootstrap;
pub mod cleanup;
pub mod globals;
pub mod subtree;
