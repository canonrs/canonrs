//! CanonRS Behaviors - CSR-only DOM behaviors

#[cfg(target_arch = "wasm32")]
pub mod behaviors;

#[cfg(target_arch = "wasm32")]
pub use behaviors::*;

#[cfg(target_arch = "wasm32")]
pub fn init() {
    behaviors::init_canonrs_behaviors();
}
