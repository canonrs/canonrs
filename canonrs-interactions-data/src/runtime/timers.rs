//! Data Runtime — timers
//! Thin wrapper over canonrs_interactions_core::runtime::timers
pub use canonrs_interactions_core::runtime::timers::{
    timeout, raf, next_frame, after_transition, after_duration,
};
pub use canonrs_interactions_core::runtime::cleanup::track_timer;
