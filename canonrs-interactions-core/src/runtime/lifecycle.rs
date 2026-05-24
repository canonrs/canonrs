//! Lifecycle — formal component lifecycle states
//!
//! States: Mount → Hydrate → Active → Replay → Suspend → Destroy
//! Every component transitions through these states explicitly.

use std::cell::RefCell;
use std::collections::HashMap;
use web_sys::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Component created, not yet initialized
    Mount,
    /// Component hydrating from SSR
    Hydrate,
    /// Component fully active with listeners
    Active,
    /// Component being replayed (remount)
    Replay,
    /// Component suspended (hidden, not destroyed)
    Suspend,
    /// Component destroyed, resources freed
    Destroy,
}

impl LifecycleState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mount   => "mount",
            Self::Hydrate => "hydrate",
            Self::Active  => "active",
            Self::Replay  => "replay",
            Self::Suspend => "suspend",
            Self::Destroy => "destroy",
        }
    }

    pub fn can_transition_to(&self, next: &LifecycleState) -> bool {
        match (self, next) {
            (Self::Mount,   Self::Hydrate) => true,
            (Self::Mount,   Self::Active)  => true,
            (Self::Hydrate, Self::Active)  => true,
            (Self::Active,  Self::Replay)  => true,
            (Self::Active,  Self::Suspend) => true,
            (Self::Active,  Self::Destroy) => true,
            (Self::Replay,  Self::Active)  => true,
            (Self::Replay,  Self::Destroy) => true,
            (Self::Suspend, Self::Active)  => true,
            (Self::Suspend, Self::Destroy) => true,
            _ => false,
        }
    }
}

thread_local! {
    static STATES: RefCell<HashMap<String, LifecycleState>> = RefCell::new(HashMap::new());
}

/// Get current lifecycle state for uid
pub fn state(uid: &str) -> Option<LifecycleState> {
    STATES.with(|s| s.borrow().get(uid).copied())
}

/// Transition to next state — returns false if transition invalid
pub fn transition(uid: &str, next: LifecycleState) -> bool {
    STATES.with(|s| {
        let mut states = s.borrow_mut();
        let current = states.get(uid).copied().unwrap_or(LifecycleState::Mount);
        if !current.can_transition_to(&next) { return false; }
        states.insert(uid.to_string(), next);
        true
    })
}

/// Force state (for bootstrap/cleanup — bypasses transition rules)
pub fn set_state(uid: &str, state: LifecycleState) {
    STATES.with(|s| { s.borrow_mut().insert(uid.to_string(), state); });
}

/// Remove uid from lifecycle registry
pub fn remove(uid: &str) {
    STATES.with(|s| { s.borrow_mut().remove(uid); });
}

/// Is component in active state
pub fn is_active(uid: &str) -> bool {
    state(uid) == Some(LifecycleState::Active)
}

/// Is component mounted (any state except Destroy)
pub fn is_mounted(uid: &str) -> bool {
    !matches!(state(uid), None | Some(LifecycleState::Destroy))
}

/// Extract uid from element
pub fn uid_of(el: &Element) -> Option<String> {
    el.get_attribute("data-rs-uid")
}
