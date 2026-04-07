//! CanonRS Interaction Engine
//! Handles complex DOM interactions that require full web APIs.
//! Islands delegate to this layer — they do not implement interactions directly.

#[cfg(target_arch = "wasm32")]
pub mod resizable;
