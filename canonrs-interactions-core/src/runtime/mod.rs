//! Runtime kernel — centralized ownership for listeners, timers and bootstrap
//!
//! Todos os crates de interação devem usar este módulo.
//! Zero cb.forget() fora deste módulo.

pub mod listeners;
pub mod timers;
pub mod bootstrap;
