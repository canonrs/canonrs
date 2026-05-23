//! Runtime kernel — centralized ownership for listeners and timers
//!
//! Todos os crates de interação devem usar este módulo.
//! Zero cb.forget() fora deste módulo.

pub mod listeners;
pub mod timers;
