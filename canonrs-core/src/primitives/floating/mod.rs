pub mod types;

#[cfg(target_arch = "wasm32")]
mod use_floating_wasm;

#[cfg(not(target_arch = "wasm32"))]
mod use_floating_stub;

#[cfg(target_arch = "wasm32")]
pub use use_floating_wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use use_floating_stub::*;
