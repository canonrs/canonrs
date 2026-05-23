//! Globals — runtime-level global state
//!
//! Observer handle, bootstrap state, GC scheduling.
//! Tudo que precisa existir uma unica vez no runtime.

use std::cell::Cell;

thread_local! {
    /// Bootstrap já executou — evita double-init no document
    static BOOTSTRAPPED: Cell<bool> = Cell::new(false);
    /// Observer já instalado — evita double-observe
    static OBSERVER_ACTIVE: Cell<bool> = Cell::new(false);
}

/// Marca bootstrap como executado — retorna false se já executou
pub fn mark_bootstrapped() -> bool {
    BOOTSTRAPPED.with(|b| {
        if b.get() { return false; }
        b.set(true);
        true
    })
}

/// Reseta bootstrap state — para testes e replay forçado
pub fn reset_bootstrap() {
    BOOTSTRAPPED.with(|b| b.set(false));
}

/// Marca observer como ativo — retorna false se já ativo
pub fn mark_observer_active() -> bool {
    OBSERVER_ACTIVE.with(|o| {
        if o.get() { return false; }
        o.set(true);
        true
    })
}

pub fn is_bootstrapped() -> bool {
    BOOTSTRAPPED.with(|b| b.get())
}

pub fn is_observer_active() -> bool {
    OBSERVER_ACTIVE.with(|o| o.get())
}
