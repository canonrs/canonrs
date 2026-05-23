//! Data Runtime — cleanup
//! Wrapper over canonrs_interactions_core::runtime::cleanup
pub use canonrs_interactions_core::runtime::cleanup::{
    cleanup_uid as run,
    cleanup_subtree,
    track_timer,
};
