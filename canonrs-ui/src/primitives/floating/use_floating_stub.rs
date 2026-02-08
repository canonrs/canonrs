//! SSR stub - no-op implementation
use leptos::prelude::*;
use super::types::FloatingConfig;

pub fn use_floating_position(
    _anchor_id: &str,
    _floating_id: &str,
    _config: FloatingConfig,
    _active: ReadSignal<bool>,
) {
    // SSR: does nothing
}
